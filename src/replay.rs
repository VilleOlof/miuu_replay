use std::fmt::Debug;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Replay {
    #[serde(rename = "typeId")]
    pub type_id: i32,
    pub version: i32,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub data: ReplayData,
}

#[derive(Deserialize)]
pub struct ReplayData {
    pub level: String,
    pub player: String,
    pub score: f64,
    pub cosmetics: ReplayCosmetics,
    #[serde(rename = "replayBuffer")]
    pub replay_buffer: Vec<u8>,
}

impl Debug for ReplayData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[derive(Debug)]
        #[allow(unused)]
        struct VisualReplayData<'a> {
            level: &'a str,
            player: &'a str,
            score: f64,
            cosmetics: &'a ReplayCosmetics,
            replay_buffer: usize,
        }

        let display_info = VisualReplayData {
            level: &self.level,
            player: &self.player,
            score: self.score,
            cosmetics: &self.cosmetics,
            replay_buffer: self.replay_buffer.len(),
        };

        if f.alternate() {
            write!(f, "{display_info:#?}")
        } else {
            write!(f, "{display_info:?}")
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ReplayCosmetics {
    pub skin: String,
    pub trail: String,
    pub respawn: String,
    pub hat: String,
    pub blast: String,
}

#[derive(Debug)]
pub struct ReplayHeader {
    pub session: i32,
    pub version: i32,
}
