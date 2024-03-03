use bevy::{
    app::{App, Startup},
    ecs::{
        system::Commands,
        schedule::{SystemSet, IntoSystemConfigs},
        component::Component,
    },
};
use foxin::render::{Layer, DrawBuffer};
use crate::systems::map::MapCameraCenter;

pub fn build(app: &mut App) {
    app.add_systems(Startup, init.in_set(SetupWindows));
}

fn init(mut commands: Commands) {
    commands.spawn((
            Layer(0),
            MapWindow,
            DrawBuffer::default(),
            MapCameraCenter::default(),
    ));
}

#[derive(SystemSet, Hash, Copy, Clone, PartialEq, Eq, Debug)]
pub struct SetupWindows;

#[derive(Component)]
pub struct MapWindow;
