SELECT users.id user_id,
    users.username,
    users.post_signature,
    topics.title
FROM topics
    INNER JOIN users ON topics.author_user_id = users.id
WHERE topics.id = ?