-- Add up migration script here
-- Permission of topic to a regular user
CREATE VIEW topic_permissions AS
SELECT
    u.id user_id,
    t.id topic_id,
    CASE
        WHEN EXISTS (
            SELECT
                1
            FROM
                moderators m
            WHERE
                m.moderator_user_id = u.id
        ) THEN 'MODERATE'
        WHEN u.id = t.author_user_id THEN 'EDIT'
        ELSE 'READ'
    END AS permission
FROM
    users u
    INNER JOIN topics t ON t.deleted_at IS NULL -- If public
    OR t.author_user_id = u.id -- If author
    OR EXISTS (
        SELECT
            1
        FROM
            moderators m
        WHERE
            m.moderator_user_id = u.id
    );

CREATE VIEW topic_public AS
SELECT
    id
FROM
    topics
WHERE
    deleted_at IS NULL;