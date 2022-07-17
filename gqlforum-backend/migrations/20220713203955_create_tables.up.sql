-- Add migration script here
-- User tables
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    last_seen TIMESTAMP,
    --
    username TEXT NOT NULL UNIQUE COLLATE NOCASE,
    phc_string TEXT,
    --
    post_signature TEXT
);
CREATE TRIGGER tr_users_after_update
AFTER
UPDATE ON users BEGIN
UPDATE users
SET updated_at = CURRENT_TIMESTAMP
WHERE users.id = NEW.id;
END;
-- Initial accounts
-- Administrator account: admin; admin
-- System announcement: system
-- General announcement: announcement
INSERT
    OR IGNORE INTO users (username, phc_string)
VALUES (
        'admin',
        '$argon2i$v=19$m=16,t=2,p=1$ZHdMaHdYeE1JZ3d6dmo0WQ$SWvpjaTUlShdvYL6qKARQg'
    ),
    ('system', NULL),
    ('announcement', NULL);
-- Forum boards
CREATE TABLE boards (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    board_name TEXT UNIQUE NOT NULL,
    creator_user_id INTEGER NOT NULL,
    FOREIGN KEY (creator_user_id) REFERENCES users(id)
);
CREATE TRIGGER tr_boards_after_update
AFTER
UPDATE ON boards BEGIN
UPDATE boards
SET updated_at = CURRENT_TIMESTAMP
WHERE boards.id = NEW.id;
END;
-- Topics
CREATE TABLE topics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,
    --
    board_id INTEGER NOT NULL,
    author_user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    FOREIGN KEY (author_user_id) REFERENCES users(id),
    FOREIGN KEY (board_id) REFERENCES boards(id)
);
CREATE TRIGGER tr_topics_after_update
AFTER
UPDATE ON topics BEGIN
UPDATE topics
SET updated_at = CURRENT_TIMESTAMP
WHERE topics.id = NEW.id;
END;
-- Posts
CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    topic_id INTEGER NOT NULL,
    author_user_id INTEGER NOT NULL,
    body TEXT NOT NULL,
    FOREIGN KEY (author_user_id) REFERENCES users(id),
    FOREIGN KEY (topic_id) REFERENCES topics(id)
);
CREATE TRIGGER tr_posts_after_update
AFTER
UPDATE ON posts BEGIN
UPDATE posts
SET updated_at = CURRENT_TIMESTAMP
WHERE posts.id = NEW.id;
END;
-- Replies
CREATE TABLE replies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    post_id INTEGER NOT NULL,
    author_user_id INTEGER NOT NULL,
    body TEXT NOT NULL
);
-- Access controls
-- Moderators
CREATE TABLE moderators (
    board_id INTEGER NOT NULL,
    moderator_user_id INTEGER NOT NULL,
    assigned_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (board_id, moderator_user_id),
    FOREIGN KEY (board_id) REFERENCES boards(id),
    FOREIGN KEY (moderator_user_id) REFERENCES user(id)
);