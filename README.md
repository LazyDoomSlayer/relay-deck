# Relay Deck

A cross-platform desktop application built with Tauri v2, Rust, Vue 3, and TypeScript for controlling Modbus RTU relay
devices.
s

## Features

- Automatic relay detection
- Connect / disconnect relay devices
- Real-time relay control
- Sequence execution
- Blink animations
- Event logging
- Cross-platform support
- Modern desktop UI

## Technology Stack

### Frontend

- Vue 3
- TypeScript
- Vite

### Backend

- Rust
- Tokio
- Tauri v2

### Communication

- Modbus RTU
- Serial Port
- RS485

## Architecture

```text
┌──────────────┐
│ Vue Frontend │
└──────┬───────┘
       │ invoke()
       ▼
┌──────────────┐
│    Tauri     │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Rust Backend │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Modbus RTU   │
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Relay Device │
└──────────────┘
```

## Hardware Used

### Waveshare Modbus RTU Relay

- 8 Relay Channels
- Modbus RTU Protocol
- RS485 Communication

### USB to RS485 Adapter

Used to connect the desktop application to the relay controller.

### Demo Device

USB-powered LED bulb connected through relay outputs.

## Example Command Flow

### Vue

```ts
await invoke("relay_set", {
    channel: 1,
    on: true,
});
```

### Rust

```rust
#[tauri::command]
pub async fn relay_set(
    state: State<'_, RelayState>,
    channel: u16,
    on: bool,
) -> Result<(), String> {
    set_coil(conn, channel, on).await
}
```

## Running

### Requirements

- Rust
- Node.js
- pnpm

### Install

```bash
pnpm install
```

### Development

```bash
pnpm tauri dev
```

### Build

```bash
pnpm tauri build
```

## Motivation

This project was created to explore how modern web technologies can be combined with Rust to build native desktop
applications capable of interacting with real hardware.

The application demonstrates:

- Frontend ↔ Rust communication
- Native desktop integrations
- Modbus RTU communication
- Hardware control using Tauri

## Bonus Project

A companion Ratatui-based terminal application was created using the same concepts and relay communication patterns:

➡️ [relay-deck-tui](https://github.com/LazyDoomSlayer/relay-deck-tui)
