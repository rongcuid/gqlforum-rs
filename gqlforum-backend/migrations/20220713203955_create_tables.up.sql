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
UPDATE
    ON users BEGIN
UPDATE
    users
SET
    updated_at = CURRENT_TIMESTAMP
WHERE
    users.id = NEW.id;

END;

-- Initial accounts
-- Administrator account: admin; admin
INSERT INTO
    users (username, phc_string)
VALUES
    (
        'admin',
        '$argon2i$v=19$m=16,t=2,p=1$ZHdMaHdYeE1JZ3d6dmo0WQ$SWvpjaTUlShdvYL6qKARQg'
    );

-- Topics
CREATE TABLE topics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,
    --
    author_user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    FOREIGN KEY (author_user_id) REFERENCES users(id)
);

CREATE TRIGGER tr_topics_after_update
AFTER
UPDATE
    ON topics BEGIN
UPDATE
    topics
SET
    updated_at = CURRENT_TIMESTAMP
WHERE
    topics.id = NEW.id;

END;

-- Posts
CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    deleted_at TIMESTAMP,
    topic_id INTEGER NOT NULL,
    author_user_id INTEGER NOT NULL,
    body TEXT NOT NULL,
    FOREIGN KEY (author_user_id) REFERENCES users(id),
    FOREIGN KEY (topic_id) REFERENCES topics(id)
);

CREATE TRIGGER tr_posts_after_update
AFTER
UPDATE
    ON posts BEGIN
UPDATE
    posts
SET
    updated_at = CURRENT_TIMESTAMP
WHERE
    posts.id = NEW.id;

END;

CREATE VIEW post_metadata AS
SELECT
    p.id AS post_id,
    t.id AS topic_id,
    ROW_NUMBER() OVER (
        PARTITION BY p.topic_id
        ORDER BY
            p.created_at
    ) AS post_number,
    p.created_at,
    p.updated_at,
    p.deleted_at
FROM
    topics t
    INNER JOIN posts p ON t.id = p.topic_id
ORDER BY
    p.created_at;

-- Sessions
CREATE TABLE active_sessions(
    session_user_id INTEGER,
    token_hash BLOB,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP,
    PRIMARY KEY(session_user_id, token_hash),
    FOREIGN KEY(session_user_id) REFERENCES users(id)
);
-- New post triggers topic update
CREATE TRIGGER tr_posts_after_insert
AFTER
INSERT
    ON posts BEGIN
UPDATE
    topics
SET
    updated_at = CURRENT_TIMESTAMP
WHERE
    topics.id = NEW.topic_id;

END;