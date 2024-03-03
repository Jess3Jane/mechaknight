use foxin::Foxin;
use bevy::{MinimalPlugins, app::App};

mod systems;
mod utils;

fn main() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, Foxin));
    systems::build(&mut app);
    app.run();
}
