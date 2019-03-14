extern crate gtk;
extern crate gio;

use gio::prelude::*;
use std::env;

mod question;
mod ui;
mod question_generator;

fn main() {
    let uiapp = gtk::Application::new("org.quiz", gio::ApplicationFlags::FLAGS_NONE).expect("Application::new failed");
    uiapp.connect_activate(|app| {
        ui::setup_ui(app);
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}
