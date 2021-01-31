#![feature(once_cell)]
mod components;
mod cache;
mod models;

use druid::widget::{Button, Flex, Label, List, CrossAxisAlignment};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, Color};
use models::State;
use crate::components::songs::build_song;
use crate::models::Song;
use crate::cache::IMAGE_CACHE;
use std::ops::Deref;

#[tokio::main]
async fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder);
    let data = State {
        count: 0_u32,
        songs: vec![
            Song {
                title: "Dark Crow".to_string(),
                artist: "MAN WITH A MISSION".to_string(),
                album: "Dark Crow".to_string(),
                image: Some("https://m.media-amazon.com/images/I/81eaclV+-1L._SS500_.jpg".to_string()),
                length: 315
            }
        ].into(),
    };
    let cache = &IMAGE_CACHE;
    cache.get_image("https://m.media-amazon.com/images/I/81eaclV+-1L._SS500_.jpg");
    AppLauncher::<State>::with_window(main_window)
        .use_simple_logger()
        .launch(data)
}

fn ui_builder() -> impl Widget<State> {
    let songs = List::new(build_song).lens(State::songs);
    let row = Flex::row().must_fill_main_axis(true)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_flex_child(sidebar_builder(), 0.3)
        .with_flex_child(songs, 0.7);
    Flex::column()
        .must_fill_main_axis(true)
        .with_flex_child(row, 1.0)
}

fn sidebar_builder() -> impl Widget<State> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &State, _env| data.count.into());
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("increment")
        .on_click(|_ctx, data: &mut State, _env| data.count += 1_u32)
        .padding(5.0);
    Flex::column()
        .must_fill_main_axis(true)
        .with_child(label)
        .with_child(button)
        .background(Color::from_hex_str("#000000").unwrap())
}

// use std::env;
// use tokio_core::reactor::Core;
//
// use librespot::core::authentication::Credentials;
// use librespot::core::config::SessionConfig;
// use librespot::core::session::Session;
// use librespot::core::spotify_id::SpotifyId;
// use librespot::playback::config::PlayerConfig;
//
// use librespot::playback::audio_backend;
// use librespot::playback::player::Player;
//
// fn main() {
//     let mut core = Core::new().unwrap();
//     let handle = core.handle();
//
//     let session_config = SessionConfig::default();
//     let player_config = PlayerConfig::default();
//
//     let args: Vec<_> = env::args().collect();
//     if args.len() != 4 {
//         println!("Usage: {} USERNAME PASSWORD TRACK", args[0]);
//     }
//     let username = args[1].to_owned();
//     let password = args[2].to_owned();
//     let credentials = Credentials::with_password(username, password);
//
//     let track = SpotifyId::from_base62(&args[3]).unwrap();
//
//     let backend = audio_backend::find(None).unwrap();
//
//     println!("Connecting ..");
//     let session = core
//         .run(Session::connect(session_config, credentials, None, handle))
//         .unwrap();
//
//     let (mut player, _) = Player::new(player_config, session.clone(), None, move || {
//         (backend)(None)
//     });
//
//     player.load(track, true, 0);
//
//     println!("Playing...");
//     core.run(player.get_end_of_track_future()).unwrap();
//
//     println!("Done");
// }