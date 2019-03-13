pub struct Question {
    text: String,
    answers: Vec<String>,
    correct_answer_index: i32
}

impl Question {
    pub fn new(text: &str) -> Question {
        Question {
            text: String::from(text),
            answers: Vec::new(),
            correct_answer_index: 0,
        }
    }

    pub fn add_answer(&mut self, text: &str) {
        self.answers.push(String::from(text));
    }

    pub fn set_correct_answer_index(&mut self, i: i32) {
        self.correct_answer_index = i;
    }

    pub fn get_text(&self) -> String {
        String::from(self.text.as_str())
    }

    pub fn get_answers(&self) -> &[String] {
        &self.answers
    }
}