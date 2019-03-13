extern crate gtk;
extern crate gio;

use gio::prelude::*;
use std::env;

mod question;
mod ui;

use question::Question;

fn main() {
    let uiapp = gtk::Application::new("org.quiz", gio::ApplicationFlags::FLAGS_NONE).expect("Application::new failed");
    uiapp.connect_activate(|app| {
        let question = get_question();
        ui::setup_ui(app, question);
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}

fn get_question() -> Question {
    let mut question = Question::new("Fourth planet?");
    question.add_answer("Jupiter");
    question.add_answer("Saturn");
    question.add_answer("Mars");
    question.add_answer("Earth");
    question.set_correct_answer_index(2);
    question
}

