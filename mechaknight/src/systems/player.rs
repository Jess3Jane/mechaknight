use bevy::{
    app::{App, Startup},
    math::IVec2,
    ecs::{
        event::EventReader,
        system::{Commands, Query, ResMut},
        component::Component,
        query::With,
    },
};
use crate::{
    utils::directions::*,
    systems::{
        world_entity::{WorldPosition, VisibleTile},
        ui_layout::MapWindow,
        map::MapCameraCenter,
    },
};
use ratatui::{
    buffer::Cell,
    style::Color,
};
use foxin::{
    schedule::{Logic, MidRender},
    input::{KeyCode, KeyPress},
    time::RenderTimeout,
};
use std::time::Instant;

pub fn build(app: &mut App) {
    app.add_systems(Startup, test_player);
    app.add_systems(Logic, walk);
    app.add_systems(MidRender, follow_player);
}

#[derive(Component, Copy, Clone)]
pub struct Player;

fn test_player(mut commands: Commands) {
    let mut cell = Cell::default();
    cell.set_char('@');
    cell.set_fg(Color::Red);
    commands.spawn((
        WorldPosition(IVec2::ZERO),
        VisibleTile(cell),
        Player,
    ));
}

fn walk(
    mut presses: EventReader<KeyPress>,
    mut render_timeout: ResMut<RenderTimeout>,
    mut player: Query<&mut WorldPosition, With<Player>>,
) {
    let mut delta = IVec2::ZERO;
    for input in presses.read() {
        if !input.modifiers.is_empty() {
            continue;
        }

        delta += match input.code {
            // Arrow keys
            KeyCode::Left      => LEFT,
            KeyCode::Right     => RIGHT,
            KeyCode::Down      => DOWN,
            KeyCode::Up        => UP,

            // Num pad diagonals
            KeyCode::PageUp    => NE,
            KeyCode::PageDown  => SE,
            KeyCode::End       => SW,
            KeyCode::Home      => NW,

            // Vim directions
            KeyCode::Char('h') => LEFT,
            KeyCode::Char('j') => DOWN,
            KeyCode::Char('k') => UP,
            KeyCode::Char('l') => RIGHT,

            // Numpad with num lock on
            KeyCode::Char('8') => N,
            KeyCode::Char('9') => NE,
            KeyCode::Char('6') => E,
            KeyCode::Char('3') => SE,
            KeyCode::Char('2') => S,
            KeyCode::Char('1') => SW,
            KeyCode::Char('4') => W,
            KeyCode::Char('7') => NW,

            _                  => IVec2::ZERO,
        };
    }

    delta = delta.signum();
    if delta == IVec2::ZERO {
        return;
    }

    for mut cur_pos in player.iter_mut() {
        let target_pos = cur_pos.0 + delta;
        cur_pos.0 = target_pos;
    }

    render_timeout.by(Instant::now());
}

fn follow_player(
    player: Query<&WorldPosition, With<Player>>,
    mut cameras: Query<&mut MapCameraCenter, With<MapWindow>>,
) {
    let pos = player.get_single().unwrap();
    for mut camera in cameras.iter_mut() {
        camera.0 = pos.0;
    }
}
