extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use std::env;

mod question;

use question::Question;

fn main() {
    let uiapp = gtk::Application::new("org.quiz", gio::ApplicationFlags::FLAGS_NONE).expect("Application::new failed");
    uiapp.connect_activate(|app| {
        let win = gtk::ApplicationWindow::new(app);
        win.set_default_size(320, 200);
        win.set_title("Quiz");

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        win.add(&vbox);

        let radio_button1 = gtk::RadioButton::new_with_label("option1");
        let radio_button2 = gtk::RadioButton::new_with_label_from_widget(&radio_button1, "option2");
        vbox.add(&radio_button1);
        vbox.add(&radio_button2);

        let button_box = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
        vbox.pack_end(&button_box, true, true, 5);

        let button = gtk::Button::new_with_label("Verify!");
        button_box.add(&button);

        clear_question(&vbox);
        let questions = get_questions();
        let radio_buttons = render_question(&questions[0], &vbox);

        button.connect_clicked(move |_| {
            verify_answer(&radio_buttons, questions[0].get_correct_answer_index());
        });

        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}

fn get_questions() -> Vec<Question> {
    let mut questions = Vec::new();
    let mut question = Question::new("Fourth planet?");
    question.add_answer("Jupiter");
    question.add_answer("Saturn");
    question.add_answer("Mars");
    question.add_answer("Earth");
    question.set_correct_answer_index(2);
    questions.push(question);
    questions
}

fn clear_question(vbox: &gtk::Box) {
    let widgets = vbox.get_children();

    for widget in widgets {
        if widget.is::<gtk::Label>() || widget.is::<gtk::RadioButton>() {
            vbox.remove(&widget);
        }
    }
}

fn render_question(question: &Question, vbox: &gtk::Box) -> Vec<gtk::RadioButton> {
    let label = gtk::Label::new(question.get_text().as_str());
    vbox.pack_start(&label, true, true, 5);
    let mut radio_buttons = Vec::new();
    let mut radio_button = Option::None;

    for (index, answer) in question.get_answers().iter().enumerate() {
        if index == 0 {
            radio_button = Option::Some(gtk::RadioButton::new_with_label(answer));
        } else {
            match radio_button {
                Some(rb) => radio_button = Some(gtk::RadioButton::new_with_label_from_widget(&rb, answer)),
                None => {},
            }
        }

        match radio_button {
            Some(ref rb) => {
                vbox.pack_start(rb, true, true, 5);
                radio_buttons.push(rb.clone());
            },
            None => {},
        }
    }

    radio_buttons
}

fn verify_answer(answers: &[gtk::RadioButton], correct_answer_index: usize) {
    if answers[correct_answer_index].get_active() {
        println!("Win!");
    } else {
        println!("Fail");
    }
}

