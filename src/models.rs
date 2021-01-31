use druid::{Data, Lens};
use druid::im::Vector;
use std::future::Future;
use druid::image::ImageBuffer;

#[derive(Clone, Data, Lens)]
pub struct State {
    pub count: u32,
    pub songs: Vector<Song>
}

#[derive(Clone, Data, Lens)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub image: Option<String>,
    pub length: u16,
}
