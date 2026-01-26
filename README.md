# nostr-arena

Python bindings for [nostr-arena](https://github.com/kako-jun/nostr-arena).

Nostr-based real-time multiplayer game arena for Python games (pygame, terminal games, etc.).

## Installation

```bash
pip install nostr-arena
```

## Quick Start

```python
from nostr_arena import Arena, ArenaConfig
import time

# Create configuration
config = ArenaConfig(
    "my-game",
    max_players=4,
    start_mode="ready"
)

# Initialize arena
arena = Arena(config)
arena.connect()

# Create a room
url = arena.create()
print(f"Share this URL: {url}")

# Game loop
while True:
    # Poll for events
    event = arena.try_recv()
    if event:
        if event["type"] == "player_join":
            print(f"Player joined: {event['player']['pubkey']}")
        elif event["type"] == "game_start":
            print("Game started!")
            break

    time.sleep(0.1)

# Send your state
arena.send_state({"score": 100, "x": 50, "y": 50})
```

## API

### ArenaConfig

```python
ArenaConfig(
    game_id: str,
    relays: list[str] | None = None,
    room_expiry: int = 600000,  # ms, 0 = permanent
    max_players: int = 2,
    start_mode: str = "auto",  # "auto", "ready", "countdown", "host"
    countdown_seconds: int = 3,
    base_url: str | None = None
)
```

### Arena

| Method | Description |
|--------|-------------|
| `connect()` | Connect to relays |
| `disconnect()` | Disconnect from relays |
| `is_connected()` | Check connection status |
| `create()` | Create a room, returns URL |
| `join(room_id)` | Join a room |
| `leave()` | Leave current room |
| `delete_room()` | Delete room (host only) |
| `send_state(state)` | Send game state (any dict) |
| `send_game_over(reason, final_score)` | Send game over |
| `send_ready(ready)` | Send ready signal |
| `start_game()` | Start game (host only) |
| `request_rematch()` | Request rematch |
| `accept_rematch()` | Accept rematch |
| `try_recv()` | Poll for event (non-blocking) |
| `players()` | Get current players |
| `player_count()` | Get player count |
| `get_room_url()` | Get room URL |
| `get_room_qr_svg()` | Get QR code as SVG |
| `get_room_qr_data_url()` | Get QR code as data URL |
| `Arena.list_rooms(game_id, relays, status, limit)` | List available rooms |

### Events

Events returned by `try_recv()` are dictionaries with a `type` field:

| Type | Fields |
|------|--------|
| `player_join` | `player` (dict with pubkey, joined_at, etc.) |
| `player_leave` | `pubkey` |
| `player_state` | `pubkey`, `state` |
| `player_disconnect` | `pubkey` |
| `player_game_over` | `pubkey`, `reason`, `final_score` |
| `rematch_requested` | `pubkey` |
| `rematch_start` | `seed` |
| `all_ready` | - |
| `countdown_start` | `seconds` |
| `countdown_tick` | `remaining` |
| `game_start` | - |
| `error` | `message` |

## Building

```bash
pip install maturin
maturin develop
```

## License

MIT
