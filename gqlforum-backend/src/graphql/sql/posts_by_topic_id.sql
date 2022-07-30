-- Select posts from a topic, bindings (current_user, topic_id, limit, offset).
-- Post contents are visible if they are not deleted or if current user is a moderator.
-- Post numbers and deletion time are always visible.
WITH meta AS (
    SELECT
        *
    FROM
        post_metadata
    WHERE
        post_metadata.topic_id = ?2
    LIMIT
        ?3 OFFSET ?4
),
content AS (
    SELECT
        p.id,
        p.author_user_id,
        p.body,
        u.username,
        u.post_signature
    FROM
        meta
        INNER JOIN posts p ON meta.post_id = p.id
        INNER JOIN users u ON p.author_user_id = u.id
    WHERE
        EXISTS (
            SELECT
                1
            FROM
                post_public ppub
            WHERE
                p.id = ppub.id
        )
        OR EXISTS (
            SELECT
                1
            FROM
                post_permissions pp
            WHERE
                p.id = pp.post_id
                AND ?1 = pp.user_id
        )
)
SELECT
    meta.post_id,
    meta.post_number,
    meta.created_at,
    meta.deleted_at,
    meta.updated_at,
    content.author_user_id,
    content.body,
    content.username,
    content.post_signature
FROM
    meta
    LEFT JOIN content ON meta.post_id = content.id;