use crate::question::Question;

pub struct QuestionGenerator {
    questions: Vec<Question>,
    next_question: usize,
}

impl QuestionGenerator {
    pub fn new() -> QuestionGenerator {
        QuestionGenerator {
            questions: create_questions(),
            next_question: 0,
        }
    }

    pub fn next(&mut self) -> Question {
        let question = self.questions[self.next_question].clone();

        if self.next_question == self.questions.len() - 1 {
            self.next_question = 0;
        } else {
            self.next_question += self.next_question;
        }

        question
    }
}

fn create_questions() -> Vec<Question> {
    let mut questions = Vec::new();

    let mut question = Question::new("Fourth planet?");
    question.add_answer("Jupiter");
    question.add_answer("Saturn");
    question.add_answer("Mars");
    question.add_answer("Earth");
    question.set_correct_answer_index(2);
    questions.push(question);

    question = Question::new("The fastest mammal?");
    question.add_answer("Horse");
    question.add_answer("Wolf");
    question.add_answer("Deer");
    question.add_answer("Cheetah");
    question.add_answer("Hare");
    question.set_correct_answer_index(3);

    questions
}
