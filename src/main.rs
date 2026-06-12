use rand::prelude::*;
use std::boxed::Box;
use wordle_tui::app::App;
use wordle_tui::words::AnswerWordPicker;

fn main() {
    let words: Vec<String> = include_str!("answer_words.txt").lines().map(|l| l.to_string()).collect();
    let rng = rand::rng();
    let mut answer_word_picker = AnswerWordPicker::new(words, Box::new(rng));
    let answer_word = answer_word_picker.get_random_word().chars().collect::<Vec<char>>().try_into().unwrap();

    let mut app = App::new(answer_word);
    ratatui::run(|terminal| app.run(terminal)).unwrap();
}
