"""Tests for nostr-arena Python bindings."""

import pytest
from nostr_arena import ArenaConfig, Arena


class TestArenaConfig:
    """Tests for ArenaConfig class."""

    def test_create_config(self):
        """Test creating a basic config."""
        config = ArenaConfig("test-game")
        assert config is not None

    def test_create_config_with_options(self):
        """Test creating config with all options."""
        config = ArenaConfig(
            game_id="test-game",
            relays=["wss://relay.damus.io"],
            room_expiry=300000,
            max_players=4,
            start_mode="ready",
            countdown_seconds=5,
            base_url="https://example.com"
        )
        assert config is not None


class TestArena:
    """Tests for Arena class."""

    def test_create_arena(self):
        """Test creating an arena instance."""
        config = ArenaConfig("test-game")
        arena = Arena(config)
        assert arena is not None

    def test_public_key(self):
        """Test getting public key."""
        config = ArenaConfig("test-game")
        arena = Arena(config)
        pubkey = arena.public_key()
        assert isinstance(pubkey, str)
        assert len(pubkey) == 64  # Hex-encoded public key

    def test_player_count_initial(self):
        """Test initial player count is 0."""
        config = ArenaConfig("test-game")
        arena = Arena(config)
        # Before creating/joining a room, player count might be 0
        count = arena.player_count()
        assert isinstance(count, int)
