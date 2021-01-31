use crate::models::{Song};
use druid::{Widget, WidgetExt, Env, Color};
use druid::widget::{Flex, Label, CrossAxisAlignment};
// use crate::components::maybe::Maybe;

pub fn build_song() -> impl Widget<Song> {
    let title = Label::new(move |song: &Song, _: &Env| format!("{}", song.title));
    let artist = Label::new(move |song: &Song, _: &Env| format!("{}", song.artist));
    let album = Label::new(move |song: &Song, _: &Env| format!("{}", song.album));
    let text = Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .must_fill_main_axis(true)
        .with_child(title)
        .with_child(artist)
        .with_child(album)
        .padding(5.0);

    let mut flex = Flex::row()
        .must_fill_main_axis(true);
    // flex.with_child(Maybe::or_empty(Image::new).lens(Song::image));

    flex.add_child(text);
    flex.background(Color::from_hex_str("#393939").unwrap())
        .rounded(5.0)
        .padding(5.0)
}