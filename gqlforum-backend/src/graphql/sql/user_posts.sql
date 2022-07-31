-- (cred_user_id, user_id, limit, offset)
SELECT
    id
FROM
    posts p
WHERE
    ?2 = p.author_user_id
    AND (
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
ORDER BY
    CASE
        WHEN p.updated_at IS NULL THEN p.created_at
        ELSE updated_at
    END DESC
LIMIT
    ?3 OFFSET ?4