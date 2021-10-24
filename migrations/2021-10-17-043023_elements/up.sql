CREATE TABLE elements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    page_id INTEGER NOT NULL,
    top REAL DEFAULT 0 NOT NULL,
    right REAL DEFAULT 0 NOT NULL,
    bottom REAL DEFAULT 0 NOT NULL,
    left REAL DEFAULT 0 NOT NULL,
    width REAL DEFAULT 0 NOT NULL,
    height REAL DEFAULT 0 NOT NULL,
    align INTEGER DEFAULT 0 NOT NULL,
    data_type INTEGER DEFAULT 0 NOT NULL,
    data TEXT DEFAULT '' NOT NULL,
    FOREIGN KEY(page_id) REFERENCES pages(id)
);