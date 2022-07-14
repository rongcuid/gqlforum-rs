-- Add migration script here

-- This is for SQLite
CREATE TABLE users (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_name TEXT UNIQUE NOT NULL,
    phc TEXT
);

CREATE TABLE boards (
    board_id INTEGER PRIMARY KEY AUTOINCREMENT,
    board_name TEXT UNIQUE NOT NULL,
    creator_user_id INTEGER NOT NULL,
    FOREIGN KEY (creator_user_id) REFERENCES users(user_id)
);

CREATE TABLE topics (
    topic_id INTEGER PRIMARY KEY AUTOINCREMENT,
    author_user_id INTEGER NOT NULL,
    board_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    FOREIGN KEY (author_user_id) REFERENCES users(user_id),
    FOREIGN KEY (board_id) REFERENCES boards(board_id)
);

CREATE TABLE posts (
    post_id INTEGER PRIMARY KEY AUTOINCREMENT,
    author_user_id INTEGER NOT NULL,
    topic_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    FOREIGN KEY (author_user_id) REFERENCES users(user_id),
    FOREIGN KEY (topic_id) REFERENCES topics(topic_id)
);
