SELECT users.id user_id,
    users.username,
    users.post_signature,
    topics.title,
    topics.created_at,
    topics.updated_at,
    topics.deleted_at
FROM topics
    INNER JOIN users ON topics.author_user_id = users.id
WHERE topics.id = ?