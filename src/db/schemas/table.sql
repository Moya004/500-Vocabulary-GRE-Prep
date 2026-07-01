CREATE TABLE vocabulary(
  word            TEXT PRIMARY KEY NOT NULL,
  class           TEXT NOT NULL,
  pronunciation   TEXT NOT NULL,
  definition      TEXT NOT NULL,
  example         TEXT NOT NULL 
);

-- Tabla virtual que indexa las columnas de texto relevantes
CREATE VIRTUAL TABLE vocabulary_fts USING fts5(
    word,
    definition,
    example,
    content='vocabulary',
    content_rowid='rowid'  -- vocabulary.word es TEXT PK, necesitas rowid implícito
);

-- Triggers para mantener el índice sincronizado
CREATE TRIGGER vocabulary_ai AFTER INSERT ON vocabulary BEGIN
    INSERT INTO vocabulary_fts(rowid, word, definition, example)
    VALUES (new.rowid, new.word, new.definition, new.example);
END;

CREATE TRIGGER vocabulary_ad AFTER DELETE ON vocabulary BEGIN
    INSERT INTO vocabulary_fts(vocabulary_fts, rowid, word, definition, example)
    VALUES ('delete', old.rowid, old.word, old.definition, old.example);
END;

CREATE TRIGGER vocabulary_au AFTER UPDATE ON vocabulary BEGIN
    INSERT INTO vocabulary_fts(vocabulary_fts, rowid, word, definition, example)
    VALUES ('delete', old.rowid, old.word, old.definition, old.example);
    INSERT INTO vocabulary_fts(rowid, word, definition, example)
    VALUES (new.rowid, new.word, new.definition, new.example);
END;
