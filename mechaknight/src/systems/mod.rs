use bevy::app::App;

macro_rules! add_modules(
    ($($module:ident)*) => {
        $(pub mod $module;)*

        pub fn build(app: &mut App) {
            $($module::build(app);)*
        }
    };
);

add_modules!(
    quit
    ui_layout
    map
    world_entity
    player
);
