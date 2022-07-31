-- topic_by_id(user_id, topic_id)
WITH meta AS (
    SELECT
        id,
        created_at,
        updated_at,
        deleted_at
    FROM
        topics
    WHERE
        id = ?2
),
content AS (
    SELECT
        t.id,
        t.author_user_id user_id,
        t.title,
        u.username,
        u.post_signature
    FROM
        meta
        INNER JOIN topics t ON meta.id = t.id
        INNER JOIN users u ON t.author_user_id = u.id
    WHERE
        EXISTS (
            SELECT
                1
            FROM
                topic_public tpub
            WHERE
                t.id = tpub.id
        )
        OR EXISTS (
            SELECT
                1
            FROM
                topic_permissions tp
            WHERE
                t.id = tp.topic_id
                AND ?1 = tp.user_id
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