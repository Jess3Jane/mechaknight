use bevy::{
    app::{App, Last},
    ecs::{
        event::EventReader,
        system::{Resource, ResMut},
    },
};

pub use bevy::app::AppExit;

pub(crate) fn build(app: &mut App) {
    app.init_resource::<ShouldQuit>();
    app.add_systems(Last, read_quit_events);
}

pub(crate) fn cleanup(_: &mut App) {}

pub(crate) fn should_quit(app: &App) -> bool {
    app.world
        .get_resource::<ShouldQuit>()
        .map(|res| res.0)
        .unwrap_or(true)
}

#[derive(Resource, Default, Debug)]
struct ShouldQuit(bool);

fn read_quit_events(
    mut should_quit: ResMut<ShouldQuit>,
    mut reader: EventReader<AppExit>,
) {
    for _ in reader.read() {
        should_quit.0 = true;
    }
}
