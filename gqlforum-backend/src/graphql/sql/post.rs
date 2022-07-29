use sqlx::{query_as, SqlitePool};

use crate::graphql::post::Post;

pub async fn query_posts_by_topic_id(
    pool: &SqlitePool,
    user_id: Option<i64>,
    topic_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Post>, sqlx::Error> {
    let posts = query_as(include_str!("posts_by_topic_id.sql"))
        .bind(user_id)
        .bind(topic_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
    Ok(posts)
}
