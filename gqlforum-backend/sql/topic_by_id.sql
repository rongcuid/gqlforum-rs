SELECT *
FROM topics INNER JOIN posts ON topics.topic_id = posts.topic_id
WHERE topics.topic_id = ?1;