#[derive(Debug, Clone)]
pub struct Word {
    pub entry: String,
    pub class: String,
    pub pronunciation: String,
    pub definition: String,
    pub example: String,
}

impl Word {
    pub fn new(a: String, b: String, c: String, d: String, e: String) -> Self {
        Word {
            entry: a,
            class: b,
            pronunciation: c,
            definition: d,
            example: e,
        }
    }
    pub fn default() -> Self {
        Self {
            entry: String::new(),
            class: String::new(),
            pronunciation: String::new(),
            definition: String::new(),
            example: String::new(),
        }
    }
}
#[derive(Debug)]
pub struct NewWordEntry {
    pub entry_input: String,
    pub class_input: String,
    pub pronunciation_input: String,
    pub definition_input: String,
    pub example_input: String,
}

impl NewWordEntry {
    pub fn new() -> Self {
        Self {
            entry_input: String::new(),
            class_input: String::new(),
            pronunciation_input: String::new(),
            definition_input: String::new(),
            example_input: String::new(),
        }
    }
}

pub enum PresentingScreen {
    Front,
    Back,
}

pub enum CurrentScreen {
    Welcome,
    Presenting(PresentingScreen),
    Adding,
    Searching,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub current_word: Option<Word>,
    pub new_word_entry: NewWordEntry,
    pub search_word_input: String,
    pub search_results: Option<Vec<Word>>,
}

impl App {
    pub fn new() -> Self {
        return Self {
            current_screen: CurrentScreen::Welcome,
            current_word: None,
            search_word_input: String::new(),
            new_word_entry: NewWordEntry::new(),
            search_results: None,
        };
    }
}
