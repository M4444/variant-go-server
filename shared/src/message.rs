use derive_more::From;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::game;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GameAction {
    Place(u32, u32),
    Pass,
    Cancel,
    BoardAt(u32, u32),
    TakeSeat(u32),
    LeaveSeat(u32),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StartGame {
    pub name: String,
    pub seats: Vec<u8>,
    pub komis: Vec<i32>,
    pub size: (u8, u8),
    pub mods: game::GameModifier,
}

#[derive(Serialize, Deserialize, Debug, Clone, From)]
pub enum ClientMessage {
    #[from(ignore)]
    Identify {
        token: Option<String>,
        nick: Option<String>,
    },
    #[from(ignore)]
    GetGameList,
    #[from(ignore)]
    JoinGame(u32),
    GameAction(GameAction),
    StartGame(StartGame),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub user_id: u64,
    pub nick: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Error {
    /// This error means the client has to wait for x seconds before it can create a game
    GameStartTimer(u64),
    Other(Cow<'static, str>),
}

impl Error {
    pub fn other(v: &'static str) -> Self {
        Error::Other(Cow::from(v))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    Identify {
        token: String,
        nick: Option<String>,
        user_id: u64,
    },
    AnnounceGame {
        room_id: u32,
        name: String,
    },
    CloseGame {
        room_id: u32,
    },
    GameStatus {
        room_id: u32,
        members: Vec<u64>,
        seats: Vec<(Option<u64>, u8)>,
        turn: u32,
        // 19x19 vec, 0 = empty, 1 = black, 2 = white
        board: Vec<u8>,
        board_visibility: Option<Vec<u16>>,
        hidden_stones_left: u32,
        size: (u8, u8),
        state: game::GameState,
        mods: game::GameModifier,
        points: Vec<i32>,
        move_number: u32,
    },
    BoardAt(game::GameHistory),
    Profile(Profile),
    MsgError(String),
    Error(Error),
}

impl ServerMessage {
    pub fn pack(&self) -> Vec<u8> {
        serde_cbor::to_vec(self).expect("cbor fail")
    }
}
