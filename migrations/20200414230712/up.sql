CREATE TABLE friendlink (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    display_text TEXT NOT NULL,
    link TEXT NOT NULL,
    show BOOLEAN NOT NULL DEFAULT 0,
    remark TEXT
);

CREATE TABLE tag (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL UNIQUE,
    display_text TEXT NOT NULL
);

CREATE TABLE category (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL UNIQUE,
    display_text TEXT NOT NULL
);

CREATE TABLE comment (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_name TEXT NOT NULL,
    email TEXT NOT NULL,
    content TEXT NOT NULL,
    comment_time TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP,'localtime')) NOT NULL,
    reply TEXT,
    reply_time TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP,'localtime')) NOT NULL,
    show BOOLEAN NOT NULL DEFAULT 0,
    foreign_id INTEGER NOT NULL,
    user_agent TEXT,
    FOREIGN KEY(foreign_id) REFERENCES post(id)
    FOREIGN KEY(foreign_id) REFERENCES page(id)
);

CREATE TABLE book (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL UNIQUE,
    display_text TEXT NOT NULL,
    description TEXT,
    cover TEXT,
    create_time TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP,'localtime')) NOT NULL
);

CREATE TABLE page (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title TEXT NOT NULL,
    url TEXT NOT NULL UNIQUE,
    raw_content TEXT NOT NULL,
    html_content TEXT NOT NULL,
    reads INTEGER DEFAULT 0 NOT NULL,
    likes INTEGER DEFAULT 0 NOT NULL,
    allow_comment BOOLEAN NOT NULL DEFAULT 0,
    published BOOLEAN NOT NULL DEFAULT TRUE,
    create_time TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP,'localtime')) NOT NULL,
    edit_time TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP,'localtime')) NOT NULL,
    parent_id INTEGER NOT NULL DEFAULT -1,
    book_id INTEGER NOT NULL,
    display_order INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY(book_id) REFERENCES book(id)
);


CREATE TABLE post (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title TEXT NOT NULL,
    url TEXT NOT NULL UNIQUE,
    raw_content TEXT NOT NULL,
    html_content TEXT NOT NULL,
    summary TEXT NOT NULL,
    thumbnail TEXT NOT NULL,
    reads INTEGER DEFAULT 0 NOT NULL,
    likes INTEGER DEFAULT 0 NOT NULL,
    allow_comment BOOLEAN NOT NULL DEFAULT 0,
    published BOOLEAN NOT NULL DEFAULT TRUE,
    create_time TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP,'localtime')) NOT NULL,
    edit_time TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP,'localtime')) NOT NULL,
    category_id INTEGER NOT NULL,
    FOREIGN KEY(category_id) REFERENCES category(id)
);

CREATE TABLE posttag (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    post_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY(post_id) REFERENCES post(id),
    FOREIGN KEY(tag_id) REFERENCES tag(id)
);

CREATE TABLE dict (
    d_key VARCHAR(30) PRIMARY KEY NOT NULL UNIQUE,
    d_value TEXT NOT NULL
);

CREATE TABLE user (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(20) NOT NULL UNIQUE,
    nick_name VARCHAR(50),
    description TEXT,
    password TEXT NOT NULL,
    avator TEXT,
    email TEXT,
    notify_comment BOOLEAN NOT NULL DEFAULT 0,
    notify_type INTEGER DEFAULT 1 NOT NULL,
    notify_email TEXT,
    session_period INTEGER DEFAULT 7200 NOT NULL
);

CREATE TABLE logininfo (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_id INTEGER,
    username TEXT NOT NULL,
    login_time TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP,'localtime')) NOT NULL,
    is_success BOOLEAN NOT NULL DEFAULT 0,
    ip TEXT,
    mac TEXT,
    user_agent TEXT
);