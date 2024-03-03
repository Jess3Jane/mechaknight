use bevy::{
    app::{App, Startup},
    ecs::{
        system::Commands,
        component::Component,
    },
};
use foxin::render::{Layer, DrawBuffer};

pub fn build(app: &mut App) {
    app.add_systems(Startup, init);
}

fn init(mut commands: Commands) {
    commands.spawn((
            Layer(0),
            MapWindow,
            DrawBuffer::default(),
    ));
}

#[derive(Component)]
pub struct MapWindow;
