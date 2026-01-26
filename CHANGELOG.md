# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2025-01-26

### Added
- Initial Python release
- Python bindings for nostr-arena core using PyO3
- All Arena methods exposed with Pythonic API
- Events returned as dictionaries
- Full type hints in `__init__.py`

### Features
- `Arena(config)` - Create arena instance
- `arena.connect()` / `arena.disconnect()` - Relay management
- `arena.create()` / `arena.join()` / `arena.leave()` - Room management
- `arena.reconnect()` - Session recovery
- `arena.send_state()` / `arena.send_ready()` / `arena.start_game()` - Game control
- `arena.try_recv()` - Non-blocking event polling
- `arena.get_room_qr_svg()` / `arena.get_room_qr_data_url()` - QR code generation
- `Arena.list_rooms()` - Room discovery

### Dependencies
- nostr-arena (Rust core via git)
- pyo3 0.23
- pythonize 0.23
- tokio (multi-thread runtime)
