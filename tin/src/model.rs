use crate::data::Set;
use std::collections::HashMap;

pub struct TinModel {
    set: Set,
    song_focused: u32,
    songs: [Option<TinSong>; 2],
}

impl TinModel {
    pub fn new(set: Set) -> Self {
        Self {
            set,
            song_focused: 0,
            songs: [None, None],
        }
    }
}

pub struct TinSong {
    matrix: TinMatrix,
}

pub struct TinMatrix {
    tracks: HashMap<i64, TinMatrixAction>,
}

pub enum TinMatrixAction {
    Playing(i64),
    Queue(i64),
    QueueTransition(i64, i64),
    Dequeue(i64),
}
