
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

insert into post (id, title, url, content, summary, thumbnail, reads, likes, allow_comment, published, category_id, create_time, edit_time) values(1, "git 学习", "learn-git",  "这是一篇学习git的文章", "如何学习git?", "https://static.liaoxuefeng.com/files/attachments/1280526512029793/l", 45, 2, 0, 1, 1,"2019-12-20 11:00:23","2019-12-20 11:00:23");
insert into post (id, title, url, content, summary, thumbnail, reads, likes, allow_comment, published, category_id, create_time, edit_time) values(2, "《堂吉诃德》", "man-of-la-mancha", "《堂吉诃德》读后感", "《堂吉诃德》", "", 120, 30, 1, 1, 2,"2020-03-20 11:00:23","2020-03-20 11:00:23");
insert into post (id, title, url, content, summary, thumbnail, reads, likes, allow_comment, published, category_id) values(3, "2020年总结", "2020-year-self-summary", "2020 是这么过得", "2020 总结", "", 45, 2, 1, 1, 3);
insert into post (id, title, url, content, summary, thumbnail, reads, likes, allow_comment, published, category_id) values(4, "golang 发送接收组播数据", "golang-multicast", "", "如何使用golang发送和接收组播数据？", "", 45, 2, 1, 1, 3);

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

insert into comment (user_name, email, content, show, foreign_id, comment_type)  values("guest", "guest@test.com","学到了！",1,4,1);
insert into comment (user_name, email, content, show, foreign_id, comment_type, reply)  values("Alan", "guest@test.com","博主，这个怎么做到的？",1,4,1,"哈哈，我也不知道呢");
insert into comment (user_name, email, content, show, foreign_id, comment_type, reply)  values("Smith", "guest@test.com","I have learned a lot, thanks",1,4,1,"you are welcome");
insert into comment (user_name, email, content, show, foreign_id, comment_type, reply)  values("Smith", "guest@test.com","I have learned a lot, thanks",1,3,1,"you are welcome");


insert into book (name, display_text, cover, description, published) values("leetcode-rs", "leetcode刷题", "", "leetcode刷题题解和总结", 1);
insert into book (name, display_text, cover, description, published) values("rust-series", "rust系列", "", "测试", 1);
insert into book (name, display_text, cover, description, published) values("rocket", "rocket框架", "", "测试rocket", 1);

insert into page (title, url, content, reads, likes, allow_comment, published, create_time, edit_time, parent_id, book_id, display_order) values("git 学习", "learn-git",  "这是一篇学习git的文章", 45, 2, 0, 1,"2019-12-20 11:00:23","2019-12-20 11:00:23", -1, 1, 0);
insert into page (title, url, content, reads, likes, allow_comment, published, create_time, edit_time, parent_id, book_id, display_order) values("《堂吉诃德》", "man-of-la-mancha", "《堂吉诃德》读后感", 120, 30, 1, 1,"2020-03-20 11:00:23","2020-03-20 11:00:23",1,1,0);
insert into page (title, url, content, reads, likes, allow_comment, published, parent_id, book_id, display_order) values("2020年总结", "2020-year-self-summary", "2020 是这么过得",45, 2, 1, 1,2,1,0);
insert into page (title, url, content, reads, likes, allow_comment, published, parent_id, book_id, display_order) values("golang 发送接收组播数据", "golang-multicast", "test test test", 45, 2, 1, 1,1,1,1);
