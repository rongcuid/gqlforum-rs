-- Add up migration script here
CREATE VIEW topic_moderators AS
SELECT t.id AS topic_id,
    m.moderator_user_id AS moderator_user_id
FROM topics t
    INNER JOIN moderators m ON t.board_id = m.board_id;