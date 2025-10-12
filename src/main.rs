mod app;
mod audio;
mod combat;
mod data;
mod ecs;
mod render;
mod states;
mod ui;
mod updater;
mod util;

#[macroquad::main("Bas Veeg Arc")]
async fn main() {
    let mut application = app::Application::new();
    application.run().await;
}
