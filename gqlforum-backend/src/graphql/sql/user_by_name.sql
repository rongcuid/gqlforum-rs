SELECT
    id,
    username,
    post_signature,
    CASE
        WHEN id = 1 THEN 'ADMINISTRATOR'
        ELSE 'REGULAR'
    END AS role
FROM
    users
WHERE
    username = ?