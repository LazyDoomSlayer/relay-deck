use tauri::State;
use tokio_modbus::prelude::*;

use std::time::Duration;
use tokio::time::sleep;

use super::state::{RelayConnection, RelayState};

const MIN_BLINK_INTERVAL_MS: u64 = 300;
const MAX_BLINK_INTERVAL_MS: u64 = 1_000;

const MIN_BLINK_TIMES: u16 = 2;
const MAX_BLINK_TIMES: u16 = 5;

/// Writes an ON/OFF value to a single relay coil.
///
/// This internal helper performs the actual Modbus RTU write operation
/// for a single relay channel. It is used by higher-level commands such
/// as `relay_set` and `relay_blink` to avoid duplicating Modbus write logic.
///
/// Channel numbering is **1-based**, while Modbus coil addressing is
/// **0-based**. This function performs the necessary conversion.
///
/// # Arguments
/// * `conn` - Mutable reference to an active relay connection
/// * `channel` - Relay channel number (1-based index)
/// * `on` - Desired relay state:
///   - `true` → turn relay **ON**
///   - `false` → turn relay **OFF**
///
/// # Returns
/// - `Ok(())` if the write operation succeeds
/// - `Err(String)` if the channel is invalid or a Modbus error occurs
///
/// # Errors
/// - Returns an error if `channel` is less than `1`
/// - Returns an error if the Modbus write fails or the device reports
///   a Modbus exception
///
/// # Notes
/// - This function does not perform any locking and assumes the caller
///   has exclusive access to the relay connection.
/// - Intended for internal use within the relay operations module.
async fn set_coil(conn: &mut RelayConnection, channel: u16, on: bool) -> Result<(), String> {
    let addr = channel.checked_sub(1).ok_or("Channel must be >= 1")?;

    conn.ctx
        .write_single_coil(addr, on)
        .await
        .map_err(|e| e.to_string())?
        .map_err(ex_to_string)?;

    Ok(())
}

/// Reads the ON/OFF state of a single relay channel (helper function).
///
/// This helper queries the relay device for the current state of the
/// specified relay channel and returns its value.
///
/// Channel numbering is **1-based**.
///
/// # Arguments
/// * `conn` - Mutable reference to an active relay connection
/// * `channel` - Relay channel number (1-based index)
///
/// # Returns
/// - `Ok(true)` if the relay channel is **ON**
/// - `Ok(false)` if the relay channel is **OFF**
/// - `Err(String)` if an error occurs
async fn read_coil(conn: &mut RelayConnection, channel: u16) -> Result<bool, String> {
    let addr = channel.checked_sub(1).ok_or("Channel must be >= 1")?;

    let values: Vec<bool> = conn
        .ctx
        .read_coils(addr, 1)
        .await
        .map_err(|e| e.to_string())?
        .map_err(ex_to_string)?;

    Ok(values.first().copied().unwrap_or(false))
}

/// Converts a Modbus exception code into a human-readable error message.
///
/// This helper is used to translate Modbus protocol–level exception
/// responses into a `String` suitable for returning from Tauri commands.
///
/// # Arguments
/// * `e` - Modbus exception code returned by the device
///
/// # Returns
/// A formatted error message describing the Modbus exception.
///
/// # Notes
/// - This function is used when a Modbus request succeeds at the
///   transport level but the device reports a protocol exception
///   (e.g. illegal address or unsupported function).
/// - The returned string is intended for logging and UI display.
fn ex_to_string(e: ExceptionCode) -> String {
    format!("Modbus exception: {:?}", e)
}

/// Reads the ON/OFF state of a single relay channel.
///
/// This command queries the relay device for the current state of the
/// specified relay channel and returns its value.
///
/// Channel numbering is **1-based**, meaning:
/// - `channel = 1` refers to the first relay
/// - `channel = 2` refers to the second relay, and so on.
///
/// # Arguments
/// * `channel` - Relay channel number (1-based index)
///
/// # Tauri
/// This function is exposed as a Tauri command and can be invoked
/// from the frontend using `invoke("relay_get")`.
///
/// # Returns
/// - `Ok(true)` if the relay channel is **ON**
/// - `Ok(false)` if the relay channel is **OFF**
/// - `Err(String)` if no relay is connected or a Modbus error occurs
///
/// # Errors
/// - Returns an error if no relay connection is active
/// - Returns an error if `channel` is less than `1`
/// - Returns an error if the Modbus read operation fails
///
/// # Notes
/// - This function performs a single Modbus RTU read request.
/// - If the device returns fewer values than requested, the relay
///   state defaults to `false`.
/// - The maximum valid channel number depends on the relay hardware.
#[tauri::command]
pub async fn relay_get(state: State<'_, RelayState>, channel: u16) -> Result<bool, String> {
    let mut guard = state.inner.lock().await;
    let conn = guard.as_mut().ok_or("Relay not connected")?;

    read_coil(conn, channel).await
}

/// Sets the ON/OFF state of a single relay channel.
///
/// This command updates the physical state of the specified relay
/// channel by issuing a Modbus RTU write operation.
///
/// Channel numbering is **1-based**, meaning:
/// - `channel = 1` controls the first relay
/// - `channel = 2` controls the second relay, and so on.
///
/// Internally, this command delegates the Modbus write logic to a
/// shared helper function to ensure consistent behavior across
/// relay operations.
///
/// # Arguments
/// * `channel` - Relay channel number (1-based index)
/// * `on` - Desired relay state:
///   - `true` → turn relay **ON**
///   - `false` → turn relay **OFF**
///
/// # Tauri
/// This function is exposed as a Tauri command and can be invoked
/// from the frontend using `invoke("relay_set")`.
///
/// # Returns
/// - `Ok(())` if the relay state is successfully updated
/// - `Err(String)` if no relay is connected or a Modbus error occurs
///
/// # Errors
/// - Returns an error if no relay connection is active
/// - Returns an error if `channel` is less than `1`
/// - Returns an error if the Modbus write operation fails or
///   the device reports a Modbus exception
///
/// # Notes
/// - The relay state change takes effect immediately.
/// - This function performs a single Modbus RTU write request.
/// - Access to the relay connection is synchronized internally.
/// - The maximum valid channel number depends on the relay hardware.
#[tauri::command]
pub async fn relay_set(state: State<'_, RelayState>, channel: u16, on: bool) -> Result<(), String> {
    let mut guard = state.inner.lock().await;
    let conn = guard.as_mut().ok_or("Relay not connected")?;

    set_coil(conn, channel, on).await
}

/// Reads the state of multiple relay channels in a single Modbus request.
///
/// This command queries the relay device for the ON/OFF state of the first
/// `count` relay channels and returns their states as a vector of booleans.
///
/// Channel indexing is **1-based** conceptually, but the returned vector
/// uses **0-based indexing**, where:
/// - index `0` corresponds to relay channel `1`
/// - index `1` corresponds to relay channel `2`, and so on.
///
/// # Arguments
/// * `count` - Number of relay channels to read starting from channel 1
///
/// # Tauri
/// This function is exposed as a Tauri command and can be invoked
/// from the frontend using `invoke("relay_get_all")`.
///
/// # Returns
/// - `Ok(Vec<bool>)` containing the relay states
/// - `Ok(Vec::new())` if `count` is `0`
/// - `Err(String)` if no relay is connected or a Modbus error occurs
///
/// # Errors
/// - Returns an error if no relay connection is active
/// - Returns an error if the Modbus request fails or the device reports
///   a Modbus exception
///
/// # Notes
/// - This function performs a single Modbus RTU request for efficiency.
/// - The maximum supported `count` depends on the relay hardware
///   (e.g. 8, 16, or 32 channels).
/// - The returned vector length is always equal to `count` on success.
#[tauri::command]
pub async fn relay_get_all(state: State<'_, RelayState>, count: u16) -> Result<Vec<bool>, String> {
    let mut guard = state.inner.lock().await;
    let conn = guard.as_mut().ok_or("Relay not connected")?;

    if count == 0 {
        return Ok(vec![]);
    }

    let values: Vec<bool> = conn
        .ctx
        .read_coils(0, count)
        .await
        .map_err(|e| e.to_string())?
        .map_err(ex_to_string)?;

    Ok(values)
}

/// Repeatedly toggles a relay channel ON and OFF with a fixed time interval,
/// then restores the relay to its original state.
///
/// This command is intended for notification-style blinking. The relay's
/// initial state is read before blinking begins and restored after the
/// blink sequence completes.
///
/// # Arguments
/// * `channel` - Relay channel number (1-based index)
/// * `times` - Number of ON/OFF cycles to perform (must be between 2 and 5)
/// * `interval_ms` - Delay in milliseconds between state changes
///   (must be between 300 ms and 1000 ms)
///
/// # Notes
/// - The relay state is restored to whatever it was before blinking began.
#[tauri::command]
pub async fn relay_blink(
    state: State<'_, RelayState>,
    channel: u16,
    times: u16,
    interval_ms: u64,
) -> Result<(), String> {
    blink_with_state(&state, channel, times, interval_ms).await
}

async fn blink_with_state(
    relay_state: &RelayState,
    channel: u16,
    times: u16,
    interval_ms: u64,
) -> Result<(), String> {
    if channel == 0 {
        return Err("Channel must be >= 1".into());
    }

    if times < MIN_BLINK_TIMES || times > MAX_BLINK_TIMES {
        return Err(format!(
            "Blink times must be between {} and {}",
            MIN_BLINK_TIMES, MAX_BLINK_TIMES
        ));
    }

    if interval_ms < MIN_BLINK_INTERVAL_MS || interval_ms > MAX_BLINK_INTERVAL_MS {
        return Err(format!(
            "Blink interval must be between {} ms and {} ms",
            MIN_BLINK_INTERVAL_MS, MAX_BLINK_INTERVAL_MS
        ));
    }

    let delay = Duration::from_millis(interval_ms);

    let initial_on: bool = {
        let mut guard = relay_state.inner.lock().await;
        let conn = guard.as_mut().ok_or("Relay not connected")?;

        let addr = channel.checked_sub(1).ok_or("Channel must be >= 1")?;

        let values: Vec<bool> = conn
            .ctx
            .read_coils(addr, 1)
            .await
            .map_err(|e| e.to_string())?
            .map_err(ex_to_string)?;

        values.first().copied().unwrap_or(false)
    };

    for _ in 0..times {
        {
            let mut guard = relay_state.inner.lock().await;
            let conn = guard.as_mut().ok_or("Relay not connected")?;
            set_coil(conn, channel, true).await?;
        }

        sleep(delay).await;

        {
            let mut guard = relay_state.inner.lock().await;
            let conn = guard.as_mut().ok_or("Relay not connected")?;
            set_coil(conn, channel, false).await?;
        }

        sleep(delay).await;
    }

    {
        let mut guard = relay_state.inner.lock().await;
        let conn = guard.as_mut().ok_or("Relay not connected")?;
        set_coil(conn, channel, initial_on).await?;
    }

    Ok(())
}
