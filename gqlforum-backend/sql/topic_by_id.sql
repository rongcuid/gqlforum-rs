-- Select posts from a topic, bindings (current_user, topic_id).
-- Post contents are visible if they are not deleted or if current user is a moderator to the topic it belongs to.
-- Post numbers and deletion time are always visible.
WITH meta AS (
    SELECT p.id AS post_id,
        ROW_NUMBER() OVER (ORDER BY p.created_at) AS post_number,
        p.deleted_at
    FROM topics t
        INNER JOIN posts p ON t.id = p.topic_id
    WHERE t.id = ?2
    ORDER BY p.created_at
), content AS (
    SELECT p.id,
        p.created_at,
        p.updated_at,
        p.author_user_id,
        u.username
    FROM posts p
        INNER JOIN users u ON p.author_user_id = u.id
    WHERE p.deleted_at NOT NULL
        OR EXISTS (
            SELECT 1
            FROM topic_moderators m
            WHERE m.topic_id = p.topic_id
                AND m.moderator_user_id = ?1
        )
)
SELECT *
FROM meta
    LEFT JOIN content;