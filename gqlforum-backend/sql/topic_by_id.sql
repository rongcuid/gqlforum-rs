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
    SELECT
		meta.post_id,
        p.created_at,
        p.updated_at,
        p.author_user_id,
		p.body,
        u.username,
        u.post_signature
    FROM meta
        INNER JOIN posts p ON meta.post_id = p.id
        INNER JOIN users u ON p.author_user_id = u.id
    WHERE p.deleted_at IS NULL
        OR EXISTS (
            SELECT 1
            FROM topic_moderators m
            WHERE m.topic_id = p.topic_id
                AND m.moderator_user_id = ?1
        )
)
SELECT 
    meta.post_id,
    meta.post_number,
    meta.deleted_at,
    content.created_at, 
    content.updated_at, 
    content.author_user_id, 
    content.body,
    content.username,
    content.post_signature
FROM meta LEFT JOIN content ON meta.post_id = content.post_id;