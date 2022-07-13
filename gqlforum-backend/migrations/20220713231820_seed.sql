-- Add migration script here
INSERT INTO users (user_name) VALUES
    ('root'),
    ('jsmith1'),
    ('test'),
    ('hacker1');

INSERT INTO boards (board_name, creator_user_id) VALUES
    ('Board A', 1),
    ('Board B', 2);

INSERT INTO topics (author_user_id, board_id, title) VALUES
    (1, 1, 'Hello, world.'),
    (2, 1, 'Message from J. Smith');

INSERT INTO posts (author_user_id, topic_id, content) VALUES
    (1, 1, 'First post on this site!'),
    (2, 1, 'Congratulations!'),
    (2, 2, 'I am John Smith.');