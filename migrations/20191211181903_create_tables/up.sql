CREATE TABLE articles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title VARCHAR NOT NULL,
    body VARCHAR NOT NULL,
    create_at TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP,'localtime')),
    published BOOLEAN NOT NULL DEFAULT 0,
    category_id INTEGER NOT NULL,
    visit_count INTEGER NOT NULL DEFAULT 0
);

INSERT INTO articles (title, body, category_id) VALUES ("Test Article1", "An Empty Article", 2);

CREATE TABLE categorys (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    label VARCHAR NOT NULL
);

INSERT INTO categorys (label) VALUES ("文章");
INSERT INTO categorys (label) VALUES ("读书");