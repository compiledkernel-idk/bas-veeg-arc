mod app;
mod audio;
mod combat;
mod coop;
mod data;
mod ecs;
mod progression;
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
