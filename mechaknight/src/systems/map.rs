use bevy::{
    app::{Startup, App},
    ecs::{component::Component, system::{Commands, Query},  query::With},
    math::{URect, U16Vec2, IVec2, IRect},
};
use ratatui::layout::Position;
use crate::{
    systems::ui_layout::MapWindow,
    utils::rect_ratatui_to_bevy,
};
use foxin::{
    schedule::Render,
    render::DrawBuffer,
};

pub fn build(app: &mut App) {
    app.add_systems(Startup, init);
    app.add_systems(Render, render_chunks);
}

fn init(mut commands: Commands) {
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
    mut buffers: Query<&mut DrawBuffer, With<MapWindow>>,
) {
    for mut buffer in buffers.iter_mut() {
        buffer.0.reset();
        for (chunk, chunk_pos) in chunks.iter() {
            let overlap = chunk_pos
                .bounds()
                .intersect(rect_ratatui_to_bevy(buffer.0.area).as_irect())
                .as_urect();
            if overlap.is_empty() { continue; }

            for x in overlap.min.x..=overlap.max.x {
                for y in overlap.min.y..=overlap.max.y {
                    let pos_in_chunk = U16Vec2 {
                        x: (x % CHUNK_SIZE.x as u32) as u16,
                        y: (y % CHUNK_SIZE.y as u32) as u16,
                    };
                    let cell = buffer.0.get_mut(x as u16, y as u16);
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

pub const CHUNK_SIZE: U16Vec2 = U16Vec2 { x: 128, y: 128 };

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
            max: min + CHUNK_SIZE.as_ivec2() - IVec2::ONE,
        }
    }
}
