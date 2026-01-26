//! Python bindings for nostr-arena

use nostr_arena::{
    Arena as CoreArena, ArenaConfig as CoreConfig, ArenaEvent as CoreEvent,
    RoomStatus, StartMode,
};
use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Arena configuration
#[pyclass]
#[derive(Clone)]
pub struct ArenaConfig {
    game_id: String,
    relays: Vec<String>,
    room_expiry: u64,
    max_players: usize,
    start_mode: StartMode,
    countdown_seconds: u32,
    base_url: Option<String>,
}

#[pymethods]
impl ArenaConfig {
    #[new]
    #[pyo3(signature = (game_id, relays=None, room_expiry=600000, max_players=2, start_mode="auto", countdown_seconds=3, base_url=None))]
    fn new(
        game_id: String,
        relays: Option<Vec<String>>,
        room_expiry: u64,
        max_players: usize,
        start_mode: &str,
        countdown_seconds: u32,
        base_url: Option<String>,
    ) -> Self {
        let mode = match start_mode {
            "auto" => StartMode::Auto,
            "ready" => StartMode::Ready,
            "countdown" => StartMode::Countdown,
            "host" => StartMode::Host,
            _ => StartMode::Auto,
        };

        Self {
            game_id,
            relays: relays.unwrap_or_else(|| vec![
                "wss://relay.damus.io".to_string(),
                "wss://nos.lol".to_string(),
                "wss://relay.nostr.band".to_string(),
            ]),
            room_expiry,
            max_players,
            start_mode: mode,
            countdown_seconds,
            base_url,
        }
    }
}

impl ArenaConfig {
    fn to_core(&self) -> CoreConfig {
        let mut config = CoreConfig::new(&self.game_id)
            .relays(self.relays.clone())
            .room_expiry(self.room_expiry)
            .max_players(self.max_players)
            .start_mode(self.start_mode.clone())
            .countdown_seconds(self.countdown_seconds);

        if let Some(ref url) = self.base_url {
            config = config.base_url(url);
        }

        config
    }
}

/// Arena - Main game room manager
#[pyclass]
pub struct Arena {
    inner: Arc<Mutex<Option<CoreArena<serde_json::Value>>>>,
    runtime: tokio::runtime::Runtime,
}

#[pymethods]
impl Arena {
    /// Create a new Arena instance
    #[new]
    fn new(config: ArenaConfig) -> PyResult<Self> {
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

        let core_config = config.to_core();
        let inner = runtime.block_on(async {
            CoreArena::new(core_config).await
        }).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

        Ok(Self {
            inner: Arc::new(Mutex::new(Some(inner))),
            runtime,
        })
    }

    /// Get public key
    fn public_key(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            Ok(arena.public_key())
        })
    }

    /// Connect to relays
    fn connect(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            arena.connect().await.map_err(|e| PyRuntimeError::new_err(e.to_string()))
        })
    }

    /// Disconnect from relays
    fn disconnect(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            arena.disconnect().await.map_err(|e| PyRuntimeError::new_err(e.to_string()))
        })
    }

    /// Check if connected
    fn is_connected(&self) -> PyResult<bool> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            Ok(arena.is_connected().await)
        })
    }

    /// Create a new room
    fn create(&self) -> PyResult<String> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            arena.create().await.map_err(|e| PyRuntimeError::new_err(e.to_string()))
        })
    }

    /// Join an existing room
    fn join(&self, room_id: &str) -> PyResult<()> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            arena.join(room_id).await.map_err(|e| PyRuntimeError::new_err(e.to_string()))
        })
    }

    /// Leave the current room
    fn leave(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            arena.leave().await.map_err(|e| PyRuntimeError::new_err(e.to_string()))
        })
    }

    /// Delete the room (host only)
    fn delete_room(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            arena.delete_room().await.map_err(|e| PyRuntimeError::new_err(e.to_string()))
        })
    }

    /// Send game state
    fn send_state(&self, state: &Bound<'_, PyAny>) -> PyResult<()> {
        let value: serde_json::Value = pythonize::depythonize(state)?;
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            arena.send_state(&value).await.map_err(|e| PyRuntimeError::new_err(e.to_string()))
        })
    }

    /// Send game over
    #[pyo3(signature = (reason, final_score=None))]
    fn send_game_over(&self, reason: &str, final_score: Option<i64>) -> PyResult<()> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            arena.send_game_over(reason, final_score).await.map_err(|e| PyRuntimeError::new_err(e.to_string()))
        })
    }

    /// Request rematch
    fn request_rematch(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            arena.request_rematch().await.map_err(|e| PyRuntimeError::new_err(e.to_string()))
        })
    }

    /// Accept rematch
    fn accept_rematch(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            arena.accept_rematch().await.map_err(|e| PyRuntimeError::new_err(e.to_string()))
        })
    }

    /// Send ready signal
    fn send_ready(&self, ready: bool) -> PyResult<()> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            arena.send_ready(ready).await.map_err(|e| PyRuntimeError::new_err(e.to_string()))
        })
    }

    /// Start game (host only)
    fn start_game(&self) -> PyResult<()> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            arena.start_game().await.map_err(|e| PyRuntimeError::new_err(e.to_string()))
        })
    }

    /// Get room URL
    fn get_room_url(&self) -> PyResult<Option<String>> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            Ok(arena.get_room_url().await)
        })
    }

    /// Get room QR code as SVG
    fn get_room_qr_svg(&self) -> PyResult<Option<String>> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            Ok(arena.get_room_qr_svg(None).await)
        })
    }

    /// Get room QR code as data URL
    fn get_room_qr_data_url(&self) -> PyResult<Option<String>> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            Ok(arena.get_room_qr_data_url(None).await)
        })
    }

    /// Get current players
    fn players(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            let players = arena.players().await;
            pythonize::pythonize(py, &players)
                .map(|b| b.unbind())
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))
        })
    }

    /// Get player count
    fn player_count(&self) -> PyResult<usize> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            Ok(arena.player_count().await)
        })
    }

    /// Poll for next event (non-blocking)
    fn try_recv(&self, py: Python<'_>) -> PyResult<Option<PyObject>> {
        self.runtime.block_on(async {
            let guard = self.inner.lock().await;
            let arena = guard.as_ref().ok_or_else(|| PyRuntimeError::new_err("Arena not initialized"))?;
            match arena.try_recv().await {
                Some(event) => {
                    let dict = event_to_dict(py, event)?;
                    Ok(Some(dict))
                }
                None => Ok(None),
            }
        })
    }

    /// List available rooms (static method)
    #[staticmethod]
    #[pyo3(signature = (game_id, relays, status=None, limit=10))]
    fn list_rooms(
        py: Python<'_>,
        game_id: &str,
        relays: Vec<String>,
        status: Option<&str>,
        limit: usize,
    ) -> PyResult<PyObject> {
        let status_filter = status.map(|s| match s {
            "waiting" => RoomStatus::Waiting,
            "playing" => RoomStatus::Playing,
            "finished" => RoomStatus::Finished,
            _ => RoomStatus::Waiting,
        });

        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

        let rooms = runtime.block_on(async {
            CoreArena::<serde_json::Value>::list_rooms(game_id, relays, status_filter, limit).await
        }).map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

        pythonize::pythonize(py, &rooms)
            .map(|b| b.unbind())
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }
}

fn event_to_dict(py: Python<'_>, event: CoreEvent<serde_json::Value>) -> PyResult<PyObject> {
    use pyo3::types::PyDict;

    let dict = PyDict::new(py);

    match event {
        CoreEvent::PlayerJoin(player) => {
            dict.set_item("type", "player_join")?;
            dict.set_item("player", pythonize::pythonize(py, &player)?.unbind())?;
        }
        CoreEvent::PlayerLeave(pubkey) => {
            dict.set_item("type", "player_leave")?;
            dict.set_item("pubkey", pubkey)?;
        }
        CoreEvent::PlayerState { pubkey, state } => {
            dict.set_item("type", "player_state")?;
            dict.set_item("pubkey", pubkey)?;
            dict.set_item("state", pythonize::pythonize(py, &state)?.unbind())?;
        }
        CoreEvent::PlayerDisconnect(pubkey) => {
            dict.set_item("type", "player_disconnect")?;
            dict.set_item("pubkey", pubkey)?;
        }
        CoreEvent::PlayerGameOver { pubkey, reason, final_score } => {
            dict.set_item("type", "player_game_over")?;
            dict.set_item("pubkey", pubkey)?;
            dict.set_item("reason", reason)?;
            dict.set_item("final_score", final_score)?;
        }
        CoreEvent::RematchRequested(pubkey) => {
            dict.set_item("type", "rematch_requested")?;
            dict.set_item("pubkey", pubkey)?;
        }
        CoreEvent::RematchStart(seed) => {
            dict.set_item("type", "rematch_start")?;
            dict.set_item("seed", seed)?;
        }
        CoreEvent::AllReady => {
            dict.set_item("type", "all_ready")?;
        }
        CoreEvent::CountdownStart(seconds) => {
            dict.set_item("type", "countdown_start")?;
            dict.set_item("seconds", seconds)?;
        }
        CoreEvent::CountdownTick(remaining) => {
            dict.set_item("type", "countdown_tick")?;
            dict.set_item("remaining", remaining)?;
        }
        CoreEvent::GameStart => {
            dict.set_item("type", "game_start")?;
        }
        CoreEvent::Error(message) => {
            dict.set_item("type", "error")?;
            dict.set_item("message", message)?;
        }
    }

    Ok(dict.unbind().into())
}

/// Python module
#[pymodule]
fn _nostr_arena(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ArenaConfig>()?;
    m.add_class::<Arena>()?;
    Ok(())
}
