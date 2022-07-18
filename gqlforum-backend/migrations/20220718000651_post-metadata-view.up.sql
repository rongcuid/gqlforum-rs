-- Add up migration script here
CREATE VIEW post_metadata
AS 
SELECT p.id AS post_id,
        t.id AS topic_id,
        ROW_NUMBER() OVER (PARTITION BY p.topic_id ORDER BY p.created_at) AS post_number,
        p.deleted_at
    FROM topics t
        INNER JOIN posts p ON t.id = p.topic_id
    ORDER BY p.created_at