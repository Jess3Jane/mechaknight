use bevy::{
    app::{App, Update},
    ecs::{
        schedule::ScheduleLabel,
        world::World,
    },
};

pub(crate) fn build(app: &mut App) {
    app.init_schedule(PreLayout);
    app.init_schedule(Layout);
    app.init_schedule(MidRender);
    app.init_schedule(Render);
    app.init_schedule(PostRender);

    app.init_schedule(PreLogic);
    app.init_schedule(Logic);
    app.init_schedule(PostLogic);

    app.add_systems(Update, run_schedule);
}

pub(crate) fn cleanup(_: &mut App) {
}

#[derive(Debug, ScheduleLabel, Hash, PartialEq, Eq, Clone)]
pub struct PreLayout;

#[derive(Debug, ScheduleLabel, Hash, PartialEq, Eq, Clone)]
pub struct Layout;

#[derive(Debug, ScheduleLabel, Hash, PartialEq, Eq, Clone)]
pub struct MidRender;

#[derive(Debug, ScheduleLabel, Hash, PartialEq, Eq, Clone)]
pub struct Render;

#[derive(Debug, ScheduleLabel, Hash, PartialEq, Eq, Clone)]
pub struct PostRender;



#[derive(Debug, ScheduleLabel, Hash, PartialEq, Eq, Clone)]
pub struct PreLogic;

#[derive(Debug, ScheduleLabel, Hash, PartialEq, Eq, Clone)]
pub struct Logic;

#[derive(Debug, ScheduleLabel, Hash, PartialEq, Eq, Clone)]
pub struct PostLogic;

fn run_schedule(world: &mut World) {
    world.run_schedule(PreLogic);
    world.run_schedule(Logic);
    world.run_schedule(PostLogic);

    if world.run_system(*crate::time::SHOULD_RENDER_SYSTEM.get().unwrap()).unwrap() {
        world.run_schedule(PreLayout);
        world.run_schedule(Layout);
        world.run_schedule(MidRender);
        world.run_schedule(Render);
        world.run_schedule(PostRender);
    }
}
