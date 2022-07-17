-- Add migration script here
DROP TRIGGER tr_users_after_update;
DROP TRIGGER tr_boards_after_update;
DROP TRIGGER tr_topics_after_update;
DROP TRIGGER tr_posts_after_update;
DROP TABLE moderators;
DROP TABLE replies;
DROP TABLE posts;
DROP TABLE topics;
DROP TABLE boards;
DROP TABLE users;