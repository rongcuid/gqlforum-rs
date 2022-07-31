-- Add migration script here
INSERT INTO topics (author_user_id, title, deleted_at)
VALUES(1, 'Website is online!', NULL),
    (1, 'Deleted', CURRENT_TIMESTAMP);

INSERT INTO posts (topic_id, author_user_id, deleted_at, body)
VALUES (1, 1, NULL, 'Hello, world.'),
    (1, 1, NULL, 'I am the admin.'),
    (1, 1, CURRENT_TIMESTAMP, 'Deleted'),
    (1, 1, NULL, 'Next post'),
    (2, 1, NULL, 'Deleted Post');