-- Add migration script here
INSERT INTO users (user_name, phc) VALUES
    ('root', null), 
    ('jsmith1', '$argon2i$v=19$m=16,t=2,p=1$YWJjZGVmZ2g$P5sf4mPlhw10x9CCTPvVcQ'), -- password
    ('test', null),
    ('hacker1', null);

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