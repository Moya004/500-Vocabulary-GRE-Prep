use crate::types::Word;
use rand::prelude::*;
use rusqlite::{Connection, Error, Result, params};

use std::path::PathBuf;

pub fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub const TABLES_SCHEMA: &str = include_str!("./schemas/table.sql");

pub struct Repository {
    connection: Connection,
}

impl Clone for Repository {
    fn clone(&self) -> Self {
        let conn = Connection::open(project_root().join("./database.sqlite3"))
            .expect("failed to clone repository connection");

        conn.execute_batch(
            "
                        PRAGMA journal_mode=WAL;
                        PRAGMA cache_size=10000;
                        PRAGMA temp_store=memory;
                        ",
        )
        .unwrap();

        Self { connection: conn }
    }
}
impl Repository {
    pub fn new() -> Option<Self> {
        let conn = Connection::open(project_root().join("./database.sqlite3"))
            .map_err(|err| println!("Error conectando a la base de datos: {}", err))
            .ok()?;

        conn.execute_batch(
            "
            PRAGMA journal_mode=WAL;
            PRAGMA cache_size=10000;
            PRAGMA temp_store=memory;
        ",
        )
        .map_err(|err| println!("Error configurando la base de datos: {}", err))
        .ok()?;

        Some(Self { connection: conn })
    }

    pub fn load_table(&self) -> Result<()> {
        let table_exists: bool = self.connection.execute(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='vocabulary'",
            (),
        )? > 0;

        if !table_exists {
            match self.connection.execute_batch(TABLES_SCHEMA) {
                Ok(()) => {}
                Err(ex) => {
                    println!("Error loading table:{}", ex);
                    return Err(ex);
                }
            };
        }

        Ok(())
    }

    pub fn add_word(&self, new_word: &Word) -> Result<(), Error> {
        let mut stm = self.connection.prepare("INSERT INTO vocabulary (word, class, pronunciation, definition, example) VALUES (?1, ?2, ?3, ?4, ?5)")?;
        match stm.execute(params![
            new_word.entry.clone(),
            new_word.class.clone(),
            new_word.pronunciation.clone(),
            new_word.definition.clone(),
            new_word.example.clone(),
        ]) {
            Ok(_) => {}
            Err(error) => {
                return {
                    println!("Error while adding a word.");
                    Err(error)
                };
            }
        }

        Ok(())
    }

    pub fn get_random_word(&self) -> Result<Word> {
        let maximum = self.connection.query_one(
            "SELECT rowid FROM vocabulary ORDER BY rowid DESC LIMIT 1",
            [],
            |row| Ok(row.get(0)?),
        )?;

        let mut rng = rand::rng();
        let id: u16 = rng.random_range(1..=maximum);
        let word = self.connection.query_one(
            "SELECT * FROM vocabulary WHERE rowid = ?1",
            params![id],
            |row| {
                Ok(Word {
                    entry: row.get(0)?,
                    class: row.get(1)?,
                    pronunciation: row.get(2)?,
                    definition: row.get(3)?,
                    example: row.get(4)?,
                })
            },
        )?;

        Ok(word)
    }

    pub fn exist_word(&self, word: &String) -> Result<bool, Error> {
        match self
            .connection
            .execute("SELECT * FROM vocabulary v WHERE v.word = ?1", [word])
        {
            Ok(res) => {
                if res > 0 {
                    return Ok(true);
                }
            }
            Err(e) => return Err(e),
        }

        Ok(false)
    }

    pub fn search_fuzzy(&self, query: String) -> Result<Vec<Word>, Error> {
        if query.trim().is_empty() {
            return Ok(vec![]);
        }

        let fts_query = query
            .split_whitespace()
            .map(|w| format!("{}*", w))
            .collect::<Vec<_>>()
            .join(" ");

        let mut stmt = self.connection.prepare_cached(
            "SELECT v.word, v.class, v.pronunciation, v.definition, v.example
             FROM vocabulary_fts f
             JOIN vocabulary v ON f.rowid = v.rowid
             WHERE vocabulary_fts MATCH ?1
             ORDER BY rank
             LIMIT 30",
        )?;

        stmt.query_map([&fts_query], |row| {
            Ok(Word {
                entry: row.get(0)?,
                class: row.get(1)?,
                pronunciation: row.get(2)?,
                definition: row.get(3)?,
                example: row.get(4)?,
            })
        })?
        .collect()
    }
}
