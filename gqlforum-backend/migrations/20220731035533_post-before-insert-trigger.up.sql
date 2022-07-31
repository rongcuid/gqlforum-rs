-- Add up migration script here
-- Check permission before posting
CREATE TRIGGER tr_posts_before_insert BEFORE
INSERT
    ON posts BEGIN
SELECT
    CASE
        WHEN NOT EXISTS (
            SELECT
                1
            FROM
                topic_permissions tp
            WHERE
                tp.topic_id = NEW.topic_id
                AND tp.user_id = NEW.author_user_id
        ) THEN RAISE (ABORT, 'Permission denied')
    END;

END;