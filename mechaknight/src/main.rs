use foxin::Foxin;
use bevy::{MinimalPlugins, app::App};
use flexi_logger::{Logger, FileSpec};

mod systems;
mod utils;

fn main() {
    Logger::try_with_env()
        .unwrap()
        .log_to_file(FileSpec::default().directory("logs"))
        .start()
        .unwrap();
    log_panics::init();
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, Foxin));
    systems::build(&mut app);
    app.run();
}
