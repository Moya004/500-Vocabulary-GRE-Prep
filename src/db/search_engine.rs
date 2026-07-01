use std::sync::mpsc::{self, Sender};
use std::thread;

use crate::db::repository::Repository;
use crate::types::Word;

pub struct SearchEngine {
    tx: Sender<Option<String>>,
}

impl SearchEngine {
    pub fn new(repo: Repository, result_tx: Sender<Vec<Word>>) -> Self {
        let (tx, rx) = mpsc::channel::<Option<String>>();

        thread::spawn(move || {
            loop {
                let mut query = match rx.recv() {
                    Ok(Some(q)) => q,
                    _ => break,
                };

                // descarta keystrokes intermedios
                while let Ok(msg) = rx.try_recv() {
                    match msg {
                        Some(q) => query = q,
                        None => return,
                    }
                }

                let results = repo.search_fuzzy(query).unwrap_or_default();
                let _ = result_tx.send(results);
            }
        });

        Self { tx }
    }

    pub fn search(&self, query: String) {
        let _ = self.tx.send(Some(query));
    }
}
