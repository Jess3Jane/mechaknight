use bevy::app::{App, Plugin};

macro_rules! add_modules(
    ($($module:ident)*) => {
        $(pub mod $module;)*

        fn build(app: &mut App) {
            $($module::build(app);)*
        }

        fn cleanup(app: &mut App) {
            $($module::cleanup(app);)*
        }
    };
);

add_modules!(
    schedule
    quit
    render
    input
    time
);

pub struct Foxin;

impl Plugin for Foxin {
    fn build(&self, app: &mut App) {
        build(app);
        app.set_runner(runner);
    }
}

fn runner(mut app: App) {
    loop {
        app.update();
        if quit::should_quit(&app) {
            break;
        }
        let max_sleep = app.world.run_system(*time::MAX_SLEEP_SYSTEM.get().unwrap()).unwrap();
        crossterm::event::poll(max_sleep).unwrap();
    }
    cleanup(&mut app);
}
