use rand::prelude::*;
use std::boxed::Box;

struct TrieNode {
    is_end: bool,
    children: [Option<Box<TrieNode>>; 26],
}
impl TrieNode {
    
    fn new() -> Self {
        Self {
            is_end: false,
            children: std::array::from_fn(|_| None),
        }
    }
    
}
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for c in word.chars() {
            let index = (c as u8 - b'a') as usize;
            node = node.children[index].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_end = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            let index = (c as u8 - b'a') as usize;
            if let Some(child) = &node.children[index] {
                node = child;
            } else {
                return false;
            }
        }
        node.is_end
    }
}

pub struct AnswerWordPicker {
    answer_words: Vec<String>,
    rng: Box<dyn Rng>,
}

impl AnswerWordPicker {
    pub fn new(answer_words: Vec<String>, rng: Box<dyn Rng>) -> Self {
        Self { answer_words, rng }
    }

    pub fn get_random_word(&mut self) -> String {
        let index = self.rng.random_range(0..self.answer_words.len());
        self.answer_words[index].clone()
    }
}