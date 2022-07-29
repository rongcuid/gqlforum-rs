SELECT
    id,
    username,
    post_signature,
    CASE
        WHEN id = 1 THEN 'ADMINISTRATOR'
        WHEN EXISTS (
            SELECT
                1
            FROM
                moderators m
            WHERE
                u.id = m.moderator_user_id
        ) THEN 'MODERATOR'
        ELSE 'REGULAR'
    END AS role
FROM
    users
WHERE
    id = ?