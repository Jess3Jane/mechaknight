use bevy::{
    app::{App, Startup, Update},
    ecs::{
        event::EventReader,
        system::{Resource, ResMut, Query, Res},
        component::Component,
        entity::Entity,
    },
    hierarchy::{Children, HierarchyPlugin},
};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::{Backend, CrosstermBackend}, layout::Rect, buffer::Buffer};
use std::{
    collections::VecDeque,
    io::{stdout, Stdout},
    time::Instant,
};

pub(crate) fn build(app: &mut App) {
    enable_raw_mode().unwrap();
    stdout().execute(EnterAlternateScreen).unwrap();
    app.insert_resource(Terminal(ratatui::Terminal::new(CrosstermBackend::new(stdout())).unwrap()));
    app.add_plugins(HierarchyPlugin);
    app.add_systems(crate::schedule::PreLayout, terminal_resize);
    app.add_systems(crate::schedule::Layout, do_layout);
    app.add_systems(crate::schedule::PostRender, do_render);
    app.add_systems(Update, redraw_on_resize); 
    app.add_systems(Startup, initial_clear);
}

pub(crate) fn cleanup(_: &mut App) {
    std::mem::drop(disable_raw_mode());
    std::mem::drop(stdout().execute(LeaveAlternateScreen));
}

#[derive(Resource)]
pub(crate) struct Terminal(pub(crate) ratatui::Terminal<CrosstermBackend<Stdout>>);

#[derive(Component, PartialOrd, Ord, PartialEq, Eq, Default)]
pub struct Layer(pub usize);

#[derive(Component, Default, Clone)]
pub struct Layout(pub ratatui::layout::Layout);

#[derive(Component, Default, Copy, Clone)]
pub struct Constraint(pub ratatui::layout::Constraint);

#[derive(Component, Default, Copy, Clone)]
pub struct DrawArea(pub Rect);

#[derive(Component, Default)]
pub struct DrawBuffer(pub Buffer);

fn redraw_on_resize(
    mut reader: EventReader<crate::input::Resize>,
    mut timeout: ResMut<crate::time::RenderTimeout>,
) {
    if !reader.is_empty() {
        timeout.by(Instant::now());
    }
}

fn terminal_resize(
    mut terminal: ResMut<Terminal>,
) {
    terminal.0.autoresize().unwrap();
}

fn initial_clear(
    mut terminal: ResMut<Terminal>,
) {
    terminal.0.clear().unwrap();
}

fn do_layout(
    terminal: Res<Terminal>,
    mut draw_areas: Query<&mut DrawArea>,
    mut draw_buffers: Query<&mut DrawBuffer>,
    constraints: Query<&Constraint>,
    layouts: Query<&Layout>,
    children: Query<&Children>,
    layers: Query<(&Layer, Entity)>,
) {
    let mut layers = layers
        .iter()
        .collect::<Vec<_>>();
    layers.sort();
    
    let area = terminal.0.size().unwrap();

    for (_, entity) in layers {
        layout_layer(
            area,
            &mut draw_areas,
            &mut draw_buffers,
            &constraints,
            &layouts,
            &children,
            entity
        );
    }
}

fn layout_layer(
    area: Rect,
    draw_areas: &mut Query<&mut DrawArea>, 
    draw_buffers: &mut Query<&mut DrawBuffer>,
    constraints: &Query<&Constraint>,
    layouts: &Query<&Layout>,
    children: &Query<&Children>,
    entity: Entity,
) {
    struct LayoutContext {
        entity: Entity,
        area: Rect,
    }
    let mut to_layout = VecDeque::new();
    to_layout.push_back(LayoutContext {
        entity,
        area,
    });

    while let Some(ctx) = to_layout.pop_front() {
        if let Ok(mut area) = draw_areas.get_mut(ctx.entity) {
            area.0 = ctx.area;
        }
        
        if let Ok(mut buffer) = draw_buffers.get_mut(ctx.entity) {
            buffer.0.resize(ctx.area);
        }

        let children = children
            .get(ctx.entity)
            .ok()
            .iter()
            .map(|v| v.iter())
            .flatten()
            .map(|entity| (entity, constraints.get(*entity).cloned().unwrap_or_default()))
            .collect::<Vec<_>>();

        let rects = layouts
            .get(ctx.entity)
            .map(|layout| layout.0.clone())
            .unwrap_or_default()
            .constraints(children.iter().map(|(_, constraint)| constraint.0))
            .split(ctx.area);

        for i in 0..children.len() {
            to_layout.push_back(LayoutContext {
                area: rects[i],
                entity: *children[i].0,
            });
        }
    }
}

fn do_render(
    mut terminal: ResMut<Terminal>,
    draw_buffers: Query<&DrawBuffer>,
    children: Query<&Children>,
    layers: Query<(&Layer, Entity)>,
) {
    let mut layers = layers
        .iter()
        .collect::<Vec<_>>();
    layers.sort();

    let mut frame = terminal.0.get_frame();

    for (_, entity) in layers.into_iter() {
        render_layer(
            frame.buffer_mut(),
            &draw_buffers, 
            &children,
            entity
        );
    }

    terminal.0.hide_cursor().unwrap();
    terminal.0.flush().unwrap();
    terminal.0.swap_buffers();
    terminal.0.backend_mut().flush().unwrap();
}

fn render_layer(
    buffer: &mut Buffer,
    draw_buffers: &Query<&DrawBuffer>,
    children: &Query<&Children>,
    entity: Entity,
) {
    let mut to_render = VecDeque::new();
    to_render.push_back(entity);

    while let Some(entity) = to_render.pop_front() {
        if let Ok(draw_buffer) = draw_buffers.get(entity) {
            buffer.merge(&draw_buffer.0);
        }

        to_render.extend(children
            .get(entity)
            .ok()
            .iter()
            .map(|v| v.iter())
            .flatten()
        );
    }
}
