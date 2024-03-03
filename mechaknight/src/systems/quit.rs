use bevy::{
    app::{App, Update},
    ecs::event::{EventReader, EventWriter},
};
use foxin::{
    input::{KeyPress, KeyCode},
    quit::AppExit,
};

pub fn build(app: &mut App) {
    app.add_systems(Update, quit_on_q);
}

fn quit_on_q(
    mut pressed: EventReader<KeyPress>,
    mut quit: EventWriter<AppExit>,
) {
    for event in pressed.read() {
        if event.code != KeyCode::Char('q') { continue; }
        if !event.modifiers.is_empty() { continue; }
        quit.send_default();
    }
}
