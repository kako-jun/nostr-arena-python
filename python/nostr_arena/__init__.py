"""
nostr-arena - Nostr-based real-time multiplayer game arena

Example:
    from nostr_arena import Arena, ArenaConfig

    config = ArenaConfig("my-game", max_players=4, start_mode="ready")
    arena = Arena(config)
    arena.connect()

    # Create a room
    url = arena.create()
    print(f"Share this URL: {url}")

    # Poll for events
    while True:
        event = arena.try_recv()
        if event:
            if event["type"] == "player_join":
                print(f"Player joined: {event['player']['pubkey']}")
            elif event["type"] == "game_start":
                print("Game started!")

        # Send your state
        arena.send_state({"score": 100, "x": 50, "y": 50})
"""

from ._nostr_arena import Arena, ArenaConfig

__all__ = ["Arena", "ArenaConfig"]
__version__ = "0.2.0"
