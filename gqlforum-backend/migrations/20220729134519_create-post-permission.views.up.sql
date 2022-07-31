-- Add up migration script here
CREATE VIEW post_permissions AS
SELECT
    u.id user_id,
    p.id post_id,
    CASE
        WHEN u.id = 1 THEN 'MODERATE'
        WHEN u.id = p.author_user_id THEN 'EDIT'
        ELSE 'READ'
    END AS permission
FROM
    users u
    JOIN posts p ON -- Is admin
    u.id = 1
    OR (
        -- Topic is visible
        EXISTS (
            SELECT
                1
            FROM
                topic_permissions tp
            WHERE
                u.id = tp.user_id
                AND p.topic_id = tp.topic_id
        )
        AND -- Is author or post is public
        (
            u.id = p.author_user_id
            OR p.deleted_at IS NULL
        )
    );

CREATE VIEW post_public AS
SELECT
    p.id
FROM
    posts p
    INNER JOIN topic_public t ON p.topic_id = t.id
WHERE
    p.deleted_at IS NULL;