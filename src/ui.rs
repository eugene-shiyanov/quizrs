use gtk::prelude::*;
use crate::question_generator::QuestionGenerator;



pub fn setup_ui(app: &gtk::Application) {
    let win = gtk::ApplicationWindow::new(app);
        win.set_default_size(320, 200);
        win.set_title("Quiz");

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        win.add(&vbox);

        let button_box = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
        vbox.pack_end(&button_box, true, true, 5);

        let button = gtk::Button::new_with_label("Start");
        button_box.add(&button);

        button.connect_clicked(move |_| {
            clear_form(&vbox);
            render_question(QuestionGenerator::new(), &vbox);
        });

        win.show_all();
}

fn clear_form(vbox: &gtk::Box) {
    for widget in vbox.get_children() {
        vbox.remove(&widget);
    }
}

fn render_question(question_generator: QuestionGenerator, vbox: &gtk::Box) {
    let mut question_generator = question_generator;
    let question = question_generator.next();
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

    let button_box = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
    vbox.pack_end(&button_box, true, true, 5);

    let button = gtk::Button::new_with_label("Verify");
    let vbox_clone = vbox.clone();
    button.connect_clicked(move |_| {
        clear_form(&vbox_clone);
        let success = verify_answer(&radio_buttons, question.get_correct_answer_index());
        render_result(&vbox_clone, success, question_generator.clone());
    });
    button_box.add(&button);
    vbox.show_all();
}

fn render_result(vbox: &gtk::Box, success: bool, question_generator: QuestionGenerator) {
    let text = if success {
        "Win!"
    } else {
        "Fail"
    };

    let label = gtk::Label::new(text);
    vbox.pack_start(&label, true, true, 5);
    let button_box = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
    vbox.pack_start(&button_box, true, true, 5);
    let button = gtk::Button::new_with_label("Start");
    let vbox_clone = vbox.clone();
    button.connect_clicked(move |_| {
        clear_form(&vbox_clone);
        render_question(question_generator.clone(), &vbox_clone);
    });
    button_box.add(&button);
    vbox.show_all();
}

fn verify_answer(answers: &[gtk::RadioButton], correct_answer_index: usize) -> bool {
    answers[correct_answer_index].get_active()
}
