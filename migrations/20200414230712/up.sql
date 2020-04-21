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
    display_text TEXT NOT NULL,
    remark TEXT NOT NULL DEFAULT "",
    weight INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE category (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL UNIQUE,
    display_text TEXT NOT NULL,
    remark TEXT NOT NULL DEFAULT ""
);

CREATE TABLE comment (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    user_name TEXT NOT NULL,
    email TEXT NOT NULL,
    raw_content TEXT NOT NULL,
    html_content TEXT NOT NULL,
    comment_time TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP,'localtime')) NOT NULL,
    reply TEXT,
    reply_time TIMESTAMP DEFAULT (datetime(CURRENT_TIMESTAMP,'localtime')) NOT NULL,
    show BOOLEAN NOT NULL DEFAULT TRUE,
    foreign_id INTEGER NOT NULL,
    comment_type INTEGER NOT NULL,
    user_agent TEXT,
    FOREIGN KEY(foreign_id) REFERENCES post(id),
    FOREIGN KEY(foreign_id) REFERENCES page(id)
);

CREATE TABLE book (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL UNIQUE,
    display_text TEXT NOT NULL,
    description TEXT,
    cover TEXT,
    published BOOLEAN NOT NULL DEFAULT FALSE,
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


-- add test data
insert into tag values(1, "github", "github", "Something about github", 1);
insert into tag values(2, "microsoft", "微软", "Something about microsoft", 1);
insert into tag values(3, "rust", "rust", "关于rust语言", 1);
insert into tag values(4, "dotnet", "dotnet", "关于.net的文章", 1);
insert into tag values(5, "java", "java", "关与java语言", 1);
insert into tag values(6, "sqlite", "sqlite", "关与sqlite数据库", 1);
insert into tag values(7, "book", "《堂吉诃德》", "读后感", 1);

insert into category values(1, "coding", "编程", "coding");
insert into category values(2, "reading", "读书", "自己读的书");
insert into category values(3, "diary", "日记", "随想吧");

insert into post (id, title, url, raw_content, html_content, summary, thumbnail, reads, likes, allow_comment, published, category_id) values(1, "git 学习", "learn-git", "这是一篇学习git的文章", "这是一篇学习git的文章", "如何学习git?", "https://static.liaoxuefeng.com/files/attachments/1280526512029793/l", 45, 2, 0, 1, 1);
insert into post (id, title, url, raw_content, html_content, summary, thumbnail, reads, likes, allow_comment, published, category_id) values(2, "《堂吉诃德》", "man-of-la-mancha", "《堂吉诃德》读后感", "《堂吉诃德》读后感", "《堂吉诃德》", "", 120, 30, 1, 1, 2);
insert into post (id, title, url, raw_content, html_content, summary, thumbnail, reads, likes, allow_comment, published, category_id) values(3, "2020年总结", "2020-year-self-summary", "2020 是这么过得", "2020 是这么过得", "2020 总结", "", 45, 2, 1, 1, 3);
insert into post (id, title, url, raw_content, html_content, summary, thumbnail, reads, likes, allow_comment, published, category_id) values(4, "golang 发送接收组播数据", "golang-multicast", "如何使用golang发送和接收组播数据？", "", "如何使用golang发送和接收组播数据？", "", 45, 2, 1, 1, 3);

insert into posttag values(1, 1, 1);
insert into posttag values(2, 1, 2);
insert into posttag values(3, 1, 3);
insert into posttag values(4, 2, 7);
insert into posttag values(5, 3, 5);
insert into posttag values(6, 3, 4);
insert into posttag values(7, 3, 6);
insert into posttag values(8, 4, 4);
insert into posttag values(9, 4, 6);

insert into dict values("index_quote", "永远相信美好的事情将要发生，祝福每个人都能健康快乐！");
insert into dict values("index_title", "Hello Yonghus's Blog");
insert into dict values("post_page_num", "10");
insert into dict values("comment_page_num", "10");
insert into dict values("about_page", "<h3>这是关于的内容</h3> <p>内容</p>");

insert into friendlink values(1, "狼煞博客", "https://blog.wolfogre.com/", 1, "老宋的博客");
insert into friendlink values(2, "老增", "https://oldzeng.com/", 1, "老方的博客");
insert into friendlink values(3, "一南向北", "https://blog.wongwongsu.com/", 1, "老王的博客");

insert into comment (user_name, email, raw_content, html_content, show, foreign_id, comment_type)  values("guest", "guest@test.com","学到了！","学到了！",1,4,1);
insert into comment (user_name, email, raw_content, html_content, show, foreign_id, comment_type, reply)  values("Alan", "guest@test.com","博主，这个怎么做到的？","博主，这个怎么做到的？",1,4,1,"哈哈，我也不知道呢");
insert into comment (user_name, email, raw_content, html_content, show, foreign_id, comment_type, reply)  values("Smith", "guest@test.com","I have learned a lot, thanks","I have learned a lot, thanks",1,4,1,"you are welcome");
insert into comment (user_name, email, raw_content, html_content, show, foreign_id, comment_type, reply)  values("Smith", "guest@test.com","I have learned a lot, thanks","I have learned a lot, thanks",1,3,1,"you are welcome");
