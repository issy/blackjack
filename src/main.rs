pub mod game;
mod tui;

fn main() {
    tui::entrypoint::start_app().unwrap();
}
