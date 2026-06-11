use super::state::RelayState;
use std::error::Error;
use tauri::State;

use tokio::time::{timeout, Duration};

use tokio_modbus::prelude::*;

use crate::relay::RelayConnection;
use tokio_modbus::client::rtu;
use tokio_serial::{DataBits, Parity, SerialStream, StopBits};

/// Creates a preconfigured serial port builder for Modbus RTU communication.
///
/// This helper returns a `SerialPortBuilder` initialized with commonly used
/// Modbus RTU parameters suitable for Waveshare and similar relay devices.
///
/// # Arguments
/// * `port_name` - Serial port identifier (e.g. `"COM5"` or `"/dev/ttyUSB0"`)
/// * `baud` - Baud rate for the serial connection (typically `9600`)
///
/// # Returns
/// A configured `SerialPortBuilder` with:
/// - 8 data bits
/// - No parity
/// - 1 stop bit
/// - A 250 ms read timeout
///
/// # Notes
/// - These settings match the default configuration of many Modbus RTU
///   relay modules.
/// - The timeout value affects detection and responsiveness and may be
///   tuned depending on the device and environment.
fn default_builder(port_name: &str, baud: u32) -> tokio_serial::SerialPortBuilder {
    tokio_serial::new(port_name, baud)
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .timeout(Duration::from_millis(250))
}

/// Determines whether a serial port should be skipped during relay detection.
///
/// This helper is used to filter out serial ports that are known to cause
/// false positives or unnecessary delays during automatic relay probing,
/// such as virtual or Bluetooth-related ports.
///
/// # Arguments
/// * `port_name` - Name of the serial port (e.g. `"COM7"`, `"/dev/ttyBluetooth"`).
///
/// # Returns
/// * `true` if the port should be skipped
/// * `false` if the port should be considered for probing
///
/// # Notes
/// - The current implementation performs a simple, case-insensitive
///   substring check.
/// - This logic is intentionally conservative and may be extended
///   or refined in the future.
fn should_skip_port(port_name: &str) -> bool {
    // Example: Windows Bluetooth virtual ports sometimes cause noise.
    let lower = port_name.to_lowercase();
    lower.contains("bluetooth")
}

/// Sends a minimal Modbus request to determine whether a slave device responds.
///
/// This function attempts to open the specified serial port and issue a
/// lightweight Modbus RTU request using the provided baud rate and slave ID.
/// Any Modbus-level response (including exception replies) is treated as a
/// successful detection.
///
/// This helper is intended **only** for relay auto-detection and must not be
/// used for establishing a persistent connection.
///
/// # Arguments
/// * `port_name` - Serial port identifier to probe
/// * `baud` - Baud rate to use when opening the serial port
/// * `slave_id` - Modbus slave address to query
///
/// # Returns
/// - `Ok(true)` if a Modbus-capable device responds on the port
/// - `Ok(false)` if no response is received *(currently unused)*
/// # Errors
/// Returns an error if the serial port cannot be opened.
///
/// # Notes
/// - Modbus exception responses are considered valid detections.
/// - The Modbus context is always disconnected before returning.
/// - This function is best-effort and does not guarantee device compatibility.
async fn probe_port(port_name: &str, baud: u32, slave_id: u8) -> Result<bool, Box<dyn Error>> {
    let builder = default_builder(port_name, baud);
    let port = SerialStream::open(&builder)?;

    let mut ctx = rtu::attach_slave(port, Slave(slave_id));

    let res = ctx.read_holding_registers(0x0000, 1).await;

    let _ = ctx.disconnect().await;

    match res {
        Ok(_) => Ok(true),
        Err(_) => Ok(true), // device responded, even if with exception
    }
}

/// Returns a list of available serial ports on the system.
///
/// This command enumerates all serial ports currently visible to the
/// operating system (e.g. `COMx` on Windows or `/dev/tty*` on Linux).
///
/// The returned list contains only the port identifiers and does not
/// perform any validation or probing of connected devices.
///
/// # Tauri
/// This function is exposed as a Tauri command and can be invoked
/// from the frontend using `invoke("list_ports")`.
///
/// # Returns
/// - `Ok(Vec<String>)` containing serial port names
/// - `Err(String)` if serial port enumeration fails
///
/// # Notes
/// - The presence of a port does not guarantee that it is usable
///   or that a relay device is connected.
/// - Use `detect_relay_connected_at_port()` to attempt automatic relay discovery.
#[tauri::command]
pub fn list_ports() -> Result<Vec<String>, String> {
    let ports = serialport::available_ports().map_err(|e| e.to_string())?;

    Ok(ports.into_iter().map(|p| p.port_name).collect::<Vec<_>>())
}

/// Attempts to automatically detect a Modbus RTU relay on available serial ports.
///
/// This command scans all available serial ports on the system and tries
/// to identify a Modbus RTU–compatible relay device using the provided
/// baud rate and slave ID.
///
/// The first port that responds to a Modbus request is returned.
/// Port scanning stops immediately after a device is detected.
///
/// # Arguments
/// * `baud` - Baud rate to use when probing serial ports (e.g. `9600`)
/// * `slave_id` - Modbus slave address to probe
///
/// # Tauri
/// This function is exposed as a Tauri command and can be invoked
/// from the frontend using `invoke("detect_relay_connected_at_port")`.
///
/// # Returns
/// - `Ok(Some(String))` containing the detected serial port name
/// - `Ok(None)` if no compatible relay device is found
/// - `Err(String)` if serial ports cannot be enumerated
///
/// # Notes
/// - Some serial ports may be skipped internally (e.g. virtual or
///   Bluetooth ports) using `should_skip_port()`.
/// - Detection is best-effort and does not guarantee full device compatibility.
/// - This function does not establish a persistent connection.
///   Use `connect_relay()` after detection.

#[tauri::command]
pub async fn detect_relay_connected_at_port(
    baud: u32,
    slave_id: u8,
) -> Result<Option<String>, String> {
    let ports = serialport::available_ports().map_err(|e| e.to_string())?;

    for p in ports {
        let port_name = p.port_name;

        if should_skip_port(&port_name) {
            continue;
        }

        match timeout(
            Duration::from_secs(2),
            probe_port(&port_name, baud, slave_id),
        )
        .await
        {
            Ok(Ok(true)) => {
                return Ok(Some(port_name));
            }
            Ok(Ok(false)) => {
                continue;
            }
            Ok(Err(error)) => {
                continue;
            }
            Err(_) => {
                continue;
            }
        }
    }

    Ok(None)
}

/// Establishes a Modbus RTU connection to a relay device.
///
/// This command opens the specified serial port using the provided
/// communication parameters and initializes a Modbus RTU context
/// bound to the given slave ID.
///
/// This function **does not perform device detection or probing**.
/// It assumes that the provided serial port and parameters are valid.
///
/// # Arguments
/// * `port_name` - Serial port identifier (e.g. `"COM5"` on Windows,
///   `"/dev/ttyUSB0"` on Linux)
/// * `baud` - Baud rate for the serial connection (typically `9600`)
/// * `slave_id` - Modbus slave address of the relay device
///
/// # Tauri
/// This function is exposed as a Tauri command and can be invoked
/// from the frontend using `invoke("connect_relay")`.
///
/// # Returns
/// - `Ok(())` if the connection is successfully established
/// - `Err(String)` if the serial port cannot be opened or initialized
///
/// # Notes
/// - Any previously active relay connection is replaced.
/// - This function does not verify relay responsiveness.
///   Use a separate probe or ping command if verification is required.
/// - After a successful call, `relay_status()` will return `Some(port_name)`.
#[tauri::command]
pub async fn connect_relay(
    state: State<'_, RelayState>,
    port_name: String,
    baud: u32,
    slave_id: u8,
) -> Result<(), String> {
    let builder = default_builder(&port_name, baud);

    let port = SerialStream::open(&builder).map_err(|e| e.to_string())?;
    let ctx = rtu::attach_slave(port, Slave(slave_id));

    let mut guard = state.inner.lock().await;
    *guard = Some(RelayConnection {
        port_name,
        baud,
        slave_id,
        ctx,
    });

    Ok(())
}

/// Disconnects from the currently connected Modbus relay.
///
/// If a relay connection exists, this command gracefully closes the
/// Modbus RTU session and releases the underlying serial port.
///
/// Calling this function when no relay is connected is safe and
/// results in a no-op.
///
/// # Tauri
/// This function is exposed as a Tauri command and can be invoked
/// from the frontend using `invoke("disconnect_relay")`.
///
/// # Returns
/// - `Ok(())` when the relay is successfully disconnected
/// - `Ok(())` if no relay was connected (no-op)
///
/// # Notes
/// - After calling this function, `relay_status()` will return `None`.
/// - The serial port becomes available for reuse by other applications.
#[tauri::command]
pub async fn disconnect_relay(state: State<'_, RelayState>) -> Result<(), String> {
    let mut guard = state.inner.lock().await;

    if let Some(conn) = guard.as_mut() {
        let _ = conn.ctx.disconnect().await;
    }

    *guard = None;
    Ok(())
}

/// Returns the current relay connection status.
///
/// If a relay is connected, this returns `Some(port_name)` where
/// `port_name` is the serial port currently in use (e.g. `"COM5"`).
///
/// If no relay is connected, this returns `None`.
///
/// # Tauri
/// This function is exposed as a Tauri command and can be invoked
/// from the frontend using `invoke("relay_status")`.
///
/// # Returns
/// - `Ok(Some(String))` if a relay connection is active
/// - `Ok(None)` if no relay is connected
/// - `Err(String)` if an internal error occurs
#[tauri::command]
pub async fn relay_status(state: State<'_, RelayState>) -> Result<Option<String>, String> {
    let guard = state.inner.lock().await;
    Ok(guard.as_ref().map(|c| c.port_name.clone()))
}
