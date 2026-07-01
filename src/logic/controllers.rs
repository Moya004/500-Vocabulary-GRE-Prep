use crate::{
    db::{repository::Repository, search_engine::SearchEngine},
    types::{App, CurrentScreen, NewWordEntry, Word},
};

use rusqlite::Error;
use std::sync::mpsc::{self, Receiver};

pub struct AppController {
    pub app_instance: App,
    pub repository: Repository,
    search_engine: SearchEngine,
    result_rx: Receiver<Vec<Word>>,
}

impl AppController {
    pub fn new() -> Self {
        let repo = Repository::new().unwrap();
        let _ = repo.load_table();
        let (result_tx, result_rx) = mpsc::channel();
        let search_engine = SearchEngine::new(repo.clone(), result_tx);

        Self {
            app_instance: App::new(),
            repository: repo,
            search_engine,
            result_rx,
        }
    }

    pub fn change_app_current_screen(&mut self, new_screen: CurrentScreen) {
        self.app_instance.current_screen = new_screen;
    }

    pub fn save_new_word(&mut self, new_word: &Word) -> Result<(), Error> {
        match self.repository.add_word(new_word) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        Ok(())
    }

    pub fn on_search_input(&mut self, c: char) {
        self.app_instance.search_word_input.push(c);
        self.search_engine
            .search(self.app_instance.search_word_input.clone());
    }

    pub fn on_search_backspace(&mut self) {
        self.app_instance.search_word_input.pop();
        self.search_engine
            .search(self.app_instance.search_word_input.clone());
    }
    pub fn search_in_dictionary(&mut self) {
        while let Ok(results) = self.result_rx.try_recv() {
            self.app_instance.search_results = Some(results);
        }
    }

    pub fn next_rand_word(&self) -> Word {
        let w = self.repository.get_random_word().unwrap();

        return w;
    }
}

pub struct NewWordEntryController;

impl NewWordEntryController {
    pub fn serve_new_word(&self, new_word_entry: &mut NewWordEntry) -> Word {
        let w = Word::new(
            new_word_entry.entry_input.clone(),
            new_word_entry.class_input.clone(),
            new_word_entry.pronunciation_input.clone(),
            new_word_entry.definition_input.clone(),
            new_word_entry.example_input.clone(),
        );

        self.reset_entry(new_word_entry);

        return w;
    }
    pub fn reset_entry(&self, new_word_entry: &mut NewWordEntry) {
        new_word_entry.entry_input.clear();
        new_word_entry.class_input.clear();
        new_word_entry.pronunciation_input.clear();
        new_word_entry.definition_input.clear();
        new_word_entry.example_input.clear();
    }
    pub fn on_word_input(&self, new_word_entry: &mut NewWordEntry, c: char) {
        new_word_entry.entry_input.push(c);
    }
    pub fn on_word_backspace(&self, new_word_entry: &mut NewWordEntry) {
        new_word_entry.entry_input.pop();
    }

    pub fn on_class_input(&self, new_word_entry: &mut NewWordEntry, c: char) {
        new_word_entry.class_input.push(c);
    }
    pub fn on_class_backspace(&self, new_word_entry: &mut NewWordEntry) {
        new_word_entry.class_input.pop();
    }

    pub fn on_punctuation_input(&self, new_word_entry: &mut NewWordEntry, c: char) {
        new_word_entry.pronunciation_input.push(c);
    }
    pub fn on_punctuation_backspace(&self, new_word_entry: &mut NewWordEntry) {
        new_word_entry.pronunciation_input.pop();
    }

    pub fn on_definition_input(&self, new_word_entry: &mut NewWordEntry, c: char) {
        new_word_entry.definition_input.push(c);
    }
    pub fn on_definition_backspace(&self, new_word_entry: &mut NewWordEntry) {
        new_word_entry.definition_input.pop();
    }
    pub fn on_example_input(&self, new_word_entry: &mut NewWordEntry, c: char) {
        new_word_entry.example_input.push(c);
    }
    pub fn on_example_backspace(&self, new_word_entry: &mut NewWordEntry) {
        new_word_entry.example_input.pop();
    }
}
