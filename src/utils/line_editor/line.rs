use std::rc::Rc;

use crate::public::env::ENV_OPTION;
use crossterm::style::Stylize;

use super::tokenizer::{tokenize, TokenVec};

// 32 ~ 125 | ' ' ~ '}'
const ALLOWED_CHAR_MAP: [bool; 95] = [
    true, // ' '
    true, // '!'
    true, // '"'
    true, // '#'
    true, // '$'
    false, false, true, // '\''
    true, // '('
    true, // ')'
    true, // '*'
    true, // '+'
    true, // ','
    true, // '-'
    true, // '.'
    true, // '/'
    true, // '0'
    true, true, true, true, true, true, true, true, true, // '9'
    true, true, // ';'
    true, // '<'
    true, // '='
    true, // '>'
    false, false, true, // 'A'
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, // 'Z'
    true, // '['
    true, // '\'
    true, // ']'
    true, // '^'
    true, // '_'
    false, true, // 'a'
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, // 'z'
    true, // '{'
    false, true, // '}'
    false,
];

pub struct Line {
    pub content: String,

    pub is_history: bool,
    pub label: String,
    pub label_width: usize,
    pub tokens: TokenVec,
}

impl Line {
    pub fn new(line_count: usize) -> Self {
        let label_str = line_count.to_string();
        let label_fmted_width = label_str.len() + 1; // `1` is space width
        let label_fmted = if unsafe { ENV_OPTION.support_ansi } {
            format!(" {}", label_str.black().on_white())
        } else {
            format!(" {}", label_str)
        };

        Line {
            content: String::new(),

            is_history: false,
            label_width: label_fmted_width,
            label: label_fmted,
            tokens: TokenVec::new(),
        }
    }

    fn refresh(&mut self) {
        // token vector refresh
        self.tokens = tokenize(&self.content);
    }
    pub fn is_allowed_char(ch: char) -> bool {
        const OFFSET: usize = 32;
        ALLOWED_CHAR_MAP[(ch as usize) - OFFSET]
    }

    // push / pop
    pub fn push(&mut self, ch: char) {
        // in: '('; pushed: "()"
        // in: '['; pushed: "[]"
        // in: '#'; pushed: '#'

        // if character is to paired
        let paired_ch = match ch {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '\'' | '\"' => ch,
            _ => '\0',
        };
        self.content.push(ch);

        // output this character with paired
        if paired_ch != '\0' {
            self.content.push(paired_ch);
        }
        self.refresh();
    }
    pub fn push_str(&mut self, str: &str) {
        self.content.push_str(str);
        self.refresh();
    }
    pub fn pop(&mut self) {
        self.content.pop();
        self.refresh();
    }

    // insert / remove
    pub fn insert(&mut self, index: usize, ch: char) -> bool {
        if Self::is_allowed_char(ch) {
            self.content.insert(index, ch);
            self.refresh();
            true
        } else {
            false
        }
    }
    pub fn remove(&mut self, index: usize) {
        self.content.remove(index);
        self.refresh();
    }

    // --- --- --- --- --- ---

    // borrow to use history content
    pub fn use_history(&mut self, content: Rc<String>) {
        self.is_history = true;
        self.tokens = tokenize(content.as_str());
    }
    // return to use self content
    pub fn reset(&mut self) {
        self.is_history = false;
        // self.tokens = tokenize(self.content);
        self.refresh();
    }
    // use history content to replace self content
    pub fn reset_with(&mut self, new_content: String) {
        self.content = new_content;
        self.is_history = false;
        self.refresh();
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }
}
