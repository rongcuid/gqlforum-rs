-- (cred_user_id, user_id, limit, offset)
SELECT
    id
FROM
    topics t
WHERE
    ?2 = t.author_user_id
    AND (
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
ORDER BY
    CASE
        WHEN t.updated_at IS NULL THEN t.created_at
        ELSE updated_at
    END DESC
LIMIT
    ?3 OFFSET ?4