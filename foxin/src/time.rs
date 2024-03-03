use bevy::{
    app::{App, First},
    ecs::system::{Resource, ResMut, SystemId, Res},
};
use std::{
    time::{Duration, Instant},
    sync::OnceLock,
};

pub(crate) static MAX_SLEEP_SYSTEM: OnceLock<SystemId<(), Duration>> = OnceLock::new(); 
pub(crate) static SHOULD_RENDER_SYSTEM: OnceLock<SystemId<(), bool>> = OnceLock::new();

pub(crate) fn build(app: &mut App) {
    app.add_systems(First, clear_logic_timeout);
    app.init_resource::<MaxRenderFrequency>();
    app.init_resource::<LastRenderTime>();
    app.init_resource::<RenderTimeout>();
    app.init_resource::<LogicTimeout>();

    MAX_SLEEP_SYSTEM.set(app.world.register_system(max_sleep)).unwrap();
    SHOULD_RENDER_SYSTEM.set(app.world.register_system(should_render)).unwrap();
}

pub(crate) fn cleanup(_: &mut App) {}

#[derive(Resource)]
pub struct MaxRenderFrequency(pub f32);

impl Default for MaxRenderFrequency {
    fn default() -> Self {
        Self(10.0)
    }
}

#[derive(Resource, Default)]
struct LastRenderTime(Option<Instant>);

fn should_render(
    mut time: ResMut<LastRenderTime>,
    mut render_timeout: ResMut<RenderTimeout>,
    max_freq: Res<MaxRenderFrequency>,
) -> bool {
    fn should_rerender(time: &Option<Instant>, timeout: &Option<Instant>, max_freq: f32) -> bool {
        let last_render_time = match time {
            Some(t) => t,
            None => {
                return true;
            }
        };

        match (max_freq, time) {
            (0.0, _) => {},
            (f, Some(t)) if t.elapsed().as_secs_f32() < 1.0 / f => {
                return false;
            },
            _ => {},
        }

        match timeout {
            Some(t) => *t < Instant::now(), 
            None => false,
        }
    }

    let should_rerender = should_rerender(&time.0, &render_timeout.0, max_freq.0);

    if should_rerender {
        time.0 = Some(Instant::now());
        render_timeout.0 = None;
    }

    should_rerender
}

#[derive(Resource, Default)]
pub struct LogicTimeout(Option<Instant>);

impl LogicTimeout {
    pub fn by(&mut self, when: Instant) {
        self.0 = Some(self.0.map(|v| v.min(when)).unwrap_or(when));
    }
}

#[derive(Resource, Default)]
pub struct RenderTimeout(Option<Instant>); 

impl RenderTimeout {
    pub fn by(&mut self, when: Instant) {
        self.0 = Some(self.0.map(|v| v.min(when)).unwrap_or(when));
    }
}

fn clear_logic_timeout(mut logic_timeout: ResMut<LogicTimeout>) {
    logic_timeout.0 = None;
}

fn max_sleep(
    render_timeout: Res<RenderTimeout>,
    logic_timeout: Res<LogicTimeout>,
    last_render_time: Res<LastRenderTime>,
    max_freq: Res<MaxRenderFrequency>,
) -> Duration {
    let mut timeout = Duration::MAX;
    let now = Instant::now();

    if let Some(t) = logic_timeout.0 {
        timeout = timeout.min(t - now); 
    }

    if let Some(t) = render_timeout.0 {
        let min_render_time = match (last_render_time.0, max_freq.0) {
            (None, _) => now,
            (Some(_), 0.0) => now,
            (Some(last), freq) => last + Duration::from_secs_f32(1.0 / freq),
        };
        
        timeout = timeout.min(t.max(min_render_time) - now);
    }

    timeout
}
