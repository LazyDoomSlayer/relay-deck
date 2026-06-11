use tokio::sync::Mutex;

/// Represents an active Modbus RTU relay connection.
///
/// This struct contains all information required to communicate with
/// a connected relay device, including the serial port configuration
/// and the Modbus RTU context.
///
/// Instances of this struct are created by `connect_relay()` and stored
/// inside `RelayState`.
pub struct RelayConnection {
    /// Name of the serial port in use (e.g. `"COM5"`, `"/dev/ttyUSB0"`).
    pub port_name: String,

    /// Baud rate used for the serial connection.
    pub baud: u32,

    /// Modbus slave address of the relay device.
    pub slave_id: u8,

    /// Active Modbus RTU client context.
    ///
    /// This context is used to perform all Modbus operations such as
    /// reading relay states or toggling outputs.
    pub ctx: tokio_modbus::client::Context,
}

/// Global application state holding the active relay connection.
///
/// This state is managed by Tauri and shared across all relay-related
/// commands. It stores the currently active relay connection, if any,
/// and provides synchronized access via an asynchronous mutex.
///
/// Only one relay connection is supported at a time.
pub struct RelayState {
    /// Inner mutable relay connection state.
    ///
    /// - `Some(RelayConnection)` when a relay is connected
    /// - `None` when no relay connection is active
    pub(crate) inner: Mutex<Option<RelayConnection>>,
}

impl RelayState {
    /// Creates a new, empty relay state.
    ///
    /// The initial state contains no active relay connection.
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(None),
        }
    }
}
