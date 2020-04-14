drop table articles;
drop table categorys;

CREATE TABLE friendlink (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    display_text TEXT NOT NULL,
    link TEXT NOT NULL,
    show BOOLEAN NOT NULL DEFAULT 0,
    remark TEXT
);

CREATE TABLE tag (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    display_text TEXT NOT NULL
);

CREATE TABLE category (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
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
    post_id INTEGER NOT NULL,
    user_agent TEXT
);

CREATE TABLE book (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    display_text TEXT NOT NULL,
    create_time TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP,'localtime')) NOT NULL
);


CREATE TABLE post (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    raw_content TEXT NOT NULL,
    html_content TEXT NOT NULL,
    summary TEXT NOT NULL,
    thumbnail TEXT NOT NULL,
    reads INTEGER DEFAULT 0 NOT NULL,
    likes INTEGER DEFAULT 0 NOT NULL,
    allow_comment BOOLEAN NOT NULL DEFAULT 0,
    create_time TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP,'localtime')) NOT NULL,
    edit_time TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP,'localtime')) NOT NULL,
    category_id INTEGER,
    post_type INTEGER NOT NULL,
    parent_id INTEGER,
    book_id INTEGER,
    display_order INTEGER
);

CREATE TABLE posttag (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    post_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL
);

CREATE TABLE dict (
    d_key VARCHAR(20) PRIMARY KEY NOT NULL UNIQUE,
    d_value TEXT NOT NULL
);

CREATE TABLE user (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(20) NOT NULL,
    nick_name VARCHAR(50),
    description TEXT,
    password TEXT NOT NULL,
    avator TEXT,
    email TEXT,
    notify_comment BOOLEAN NOT NULL DEFAULT 0,
    notify_type INTEGER DEFAULT 1,
    notify_email TEXT,
    session_period INTEGER DEFAULT 7200
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