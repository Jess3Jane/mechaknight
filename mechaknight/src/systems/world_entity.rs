use bevy::{
    app::{App, Startup},
    ecs::{
        component::Component,
        system::{Query, Commands},
        schedule::IntoSystemConfigs,
    },
    math::IVec2,
};
use crate::systems::map::{MapRender, MapCameraCenter};
use foxin::{
    render::DrawBuffer,
    schedule::Render,
};
use ratatui::{
    buffer::Cell,
    style::Color,
};

pub fn build(app: &mut App) {
    app.add_systems(Render, render_tiles.after(MapRender));
    app.add_systems(Startup, test_ents);
}

#[derive(Component, Debug, Copy, Clone)]
pub struct WorldPosition(pub IVec2);

#[derive(Component, Debug, Clone, Default)]
pub struct VisibleTile(pub Cell);

fn render_tiles(
    tiles: Query<(&WorldPosition, &VisibleTile)>,
    mut buffers: Query<(&mut DrawBuffer, &MapCameraCenter)>,
) {
    for (mut buffer, camera_center) in buffers.iter_mut() {
        let bounds = camera_center.get_view_rect(&buffer);
        let view_offset = camera_center.get_view_offset(&buffer);
        for (pos, tile) in tiles.iter() {
            if !bounds.contains(pos.0) { continue; }
            let cell_pos = (pos.0 + view_offset).as_u16vec2();
            *buffer.0.get_mut(cell_pos.x, cell_pos.y) = tile.0.clone();
        }
    }
}

fn test_ents(mut commands: Commands) {
    let mut cell = Cell::default();
    cell.set_char('M');
    cell.set_fg(Color::Yellow);
    commands.spawn((
            WorldPosition(IVec2::ONE * 25),
            VisibleTile(cell),
    ));
}
