use super::super::app::{ActiveBlock, App, RouteId};
use termion::event::Key;

pub fn down_event(key: Key) -> bool {
    match key {
        Key::Down | Key::Char('j') | Key::Ctrl('n') => true,
        _ => false,
    }
}

pub fn up_event(key: Key) -> bool {
    match key {
        Key::Up | Key::Char('k') | Key::Ctrl('p') => true,
        _ => false,
    }
}

pub fn left_event(key: Key) -> bool {
    match key {
        Key::Left | Key::Char('h') => true,
        _ => false,
    }
}

pub fn right_event(key: Key) -> bool {
    match key {
        Key::Right | Key::Char('l') => true,
        _ => false,
    }
}

pub fn on_down_press_handler<T>(selection_data: &[T], selection_index: Option<usize>) -> usize {
    match selection_index {
        Some(selection_index) => {
            if !selection_data.is_empty() {
                let next_index = selection_index + 1;
                if next_index > selection_data.len() - 1 {
                    return 0;
                } else {
                    return next_index;
                }
            }
            0
        }
        None => 0,
    }
}

pub fn on_up_press_handler<T>(selection_data: &[T], selection_index: Option<usize>) -> usize {
    match selection_index {
        Some(selection_index) => {
            if !selection_data.is_empty() {
                if selection_index > 0 {
                    return selection_index - 1;
                } else {
                    return selection_data.len() - 1;
                }
            }
            0
        }
        None => 0,
    }
}

pub fn handle_left_event(app: &mut App) {
    app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Recommend));
}

// handle right key event
pub fn handle_right_event(app: &mut App) {
    match app.get_current_route().hovered_block {
        ActiveBlock::MyPlaylists | ActiveBlock::Recommend => match app.get_current_route().id {
            RouteId::TrackTable => {
                app.set_current_route_state(
                    Some(ActiveBlock::Empty),
                    Some(ActiveBlock::TrackTable),
                );
            }
            RouteId::Search => {
                app.set_current_route_state(
                    Some(ActiveBlock::Empty),
                    Some(ActiveBlock::SearchResult),
                );
            }
            RouteId::AlbumList => {
                app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::AlbumList));
            }
            RouteId::Artist => {
                app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Artist));
            }
            RouteId::Playlist => {
                app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Playlist));
            }
            RouteId::ArtistList => {
                app.set_current_route_state(
                    Some(ActiveBlock::Empty),
                    Some(ActiveBlock::ArtistList),
                );
            }
            RouteId::PersonalFm => {
                app.set_current_route_state(
                    Some(ActiveBlock::Empty),
                    Some(ActiveBlock::PersonalFm),
                );
            }
            RouteId::AlbumTracks => {
                app.set_current_route_state(
                    Some(ActiveBlock::Empty),
                    Some(ActiveBlock::AlbumTracks),
                );
            }
            RouteId::Playing => {
                app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Playing));
            }
            RouteId::DjRadio => {
                app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::DjRadio));
            }
            RouteId::DjProgram => {
                app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::DjProgram));
            }
            RouteId::Home => {
                app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Home));
            }
            RouteId::Error => {}
            _ => {}
        },
        _ => {}
    };
}
