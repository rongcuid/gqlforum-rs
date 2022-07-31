-- post_by_id(user_id, post_id)
WITH meta AS (
    SELECT
        id,
        created_at,
        updated_at,
        deleted_at
    FROM
        posts
    WHERE
        id = ?2
),
content AS (
    SELECT
        p.id,
        p.topic_id,
        p.author_user_id user_id,
        p.body,
        u.username,
        u.post_signature
    FROM
        meta
        INNER JOIN posts p ON meta.id = p.id
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
    content.*,
    meta.created_at,
    meta.updated_at,
    meta.deleted_at
FROM
    meta
    LEFT JOIN content ON meta.id = content.id