use bevy::{
    app::{Startup, App},
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query}, 
        query::With,
        schedule::IntoSystemConfigs,
    },
    math::{URect, U16Vec2, IVec2, IRect},
};
use ratatui::layout::Position;
use crate::{
    systems::ui_layout::{SetupWindows, MapWindow},
    utils::rect_ratatui_to_bevy,
};
use foxin::{
    schedule::Render,
    render::DrawBuffer,
};
use log::{debug, trace};

pub fn build(app: &mut App) {
    app.add_systems(Startup, (test_chunks));
    app.add_systems(Render, render_chunks);
}

fn test_chunks(mut commands: Commands) {
    for x in 0..10 {
        for y in 0..10 {
            commands.spawn((
                ChunkData::gen_quads(),
                ChunkPosition(IVec2 { x, y }),
            ));
        }
    }
}

fn render_chunks(
    chunks: Query<(&ChunkData, &ChunkPosition)>,
    mut buffers: Query<(&mut DrawBuffer, &MapCameraCenter), With<MapWindow>>,
) {
    for (mut buffer, camera_center) in buffers.iter_mut() {
        let map_bounds = camera_center.get_view_rect(&buffer);
        let view_offset = camera_center.get_view_offset(&buffer);
        debug!("Map view rect: {:#?}", map_bounds);
        debug!("Map view offset: {:#?}", view_offset);
        buffer.0.reset();
        for (chunk, chunk_pos) in chunks.iter() {
            let overlap = chunk_pos
                .bounds()
                .intersect(map_bounds);
            if overlap.is_empty() { continue; }
            debug!("Overlap for chunk {:?}: {:#?}", chunk_pos, overlap);
            for x in 0..overlap.width() {
                for y in 0..overlap.height() {
                    let delta = IVec2 { x, y };
                    let cell_pos = overlap.min + delta;
                    let pos_in_chunk = (cell_pos % CHUNK_SIZE.as_ivec2()).as_u16vec2();
                    let buffer_pos = (cell_pos + view_offset).as_u16vec2();
                    let cell = buffer.0.get_mut(buffer_pos.x, buffer_pos.y);
                    match chunk.data[ChunkData::get_index(pos_in_chunk)] {
                        Tile::Floor => {
                            cell.set_char('.');
                        }
                        Tile::Wall => {
                            cell.set_char('#');
                        },
                    }
                }
            }
        }
    }
}

pub const CHUNK_SIZE: U16Vec2 = U16Vec2 { x: 4, y: 4 };

#[derive(Copy, Clone, Debug, Default,)]
pub enum Tile {
    #[default]
    Floor,
    Wall,
}

#[derive(Component, Clone)]
pub struct ChunkData {
    pub data: [Tile; (CHUNK_SIZE.x * CHUNK_SIZE.y) as usize],
}

impl Default for ChunkData {
    fn default() -> Self {
        Self {
            data: [Tile::default(); (CHUNK_SIZE.x * CHUNK_SIZE.y) as usize],
        }
    }
}

impl ChunkData {
    pub const fn get_index(pos: U16Vec2) -> usize {
        (pos.x + pos.y * CHUNK_SIZE.x) as usize
    }

    pub fn gen_checkerboard() -> Self {
        let mut this = Self::default();
        for i in 0..this.data.len() {
            this.data[i] = [
                Tile::Floor,
                Tile::Wall,
            ][i % 2];
        }
        this
    }
    pub fn gen_quads() -> Self {
        let mut this = Self::default();
        for x in 0..CHUNK_SIZE.x {
            for y in 0..CHUNK_SIZE.y {
                let i = Self::get_index(U16Vec2 { x, y });
                let v = (2 * x / CHUNK_SIZE.x) + (2 * y / CHUNK_SIZE.y);
                this.data[i] = [
                    Tile::Floor,
                    Tile::Wall,
                ][v as usize % 2];
            }
        }
        this
    }
}

#[derive(Component, Clone, Debug)]
pub struct ChunkPosition(pub IVec2);

impl ChunkPosition {
    pub fn bounds(&self) -> IRect {
        let min = IVec2 {
            x: self.0.x * CHUNK_SIZE.x as i32,
            y: self.0.y * CHUNK_SIZE.y as i32,
        };
        IRect {
            min,
            max: min + CHUNK_SIZE.as_ivec2(),
        }
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct MapCameraCenter(pub IVec2);

impl MapCameraCenter {
    pub fn get_view_rect(&self, buffer: &DrawBuffer) -> IRect {
        let size = buffer.0.area.as_size();
        IRect::from_center_size(
            self.0,
            IVec2 { x: size.width as i32, y: size.height as i32, }
        )
    }
    
    /// Get a vector translating world coordinates into buffer coordinates
    pub fn get_view_offset(&self, buffer: &DrawBuffer) -> IVec2 {
        let view_rect = self.get_view_rect(buffer);
        IVec2 {
            x: buffer.0.area.x as i32 - view_rect.min.x,
            y: buffer.0.area.y as i32 - view_rect.min.y,
        }
    }
}
