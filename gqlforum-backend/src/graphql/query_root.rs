use async_graphql::*;
use tracing::debug;

pub struct QueryRoot;

use super::topics;

#[Object]
impl QueryRoot {
    // async fn board(&self, _ctx: &Context<'_>, _id: i64) -> Option<top_down::Board> {
    //     None
    // }
    async fn topic(&self, ctx: &Context<'_>, user_id: i64, topic_id: i64) -> Option<topics::Topic> {
        None
        // let pool = ctx.data::<SqlitePool>().unwrap();
        // debug!("Querying for topics");
        // let mut tx = pool
        //     .begin()
        //     .await
        //     .expect("Failed to begin transaction for `topic`");
        // let meta = query_file!("sql/topic_meta.sql", topic_id)
        //     .fetch_optional(&mut tx)
        //     .await
        //     .expect("Failed on `topic` select topic metadata")?;
        // let posts = if ctx.look_ahead().field("posts").exists() {
        //     debug!("Querying for posts");
        //     let posts = query_file!("sql/topic_by_id.sql", user_id, topic_id)
        //         .map(|row| {
        //             let f = || -> Option<topics::Author> {
        //                 Some(topics::Author {
        //                     id: row.author_user_id?,
        //                     name: row.username?,
        //                     signature: row.post_signature,
        //                 })
        //             };
        //             let author: Option<topics::Author> = f();
        //             topics::Post {
        //                 post_number: row.post_number,
        //                 deleted_at: row.deleted_at,
        //                 author,
        //                 body: row.body,
        //             }
        //         })
        //         .fetch_all(&mut tx)
        //         .await
        //         .expect("Failed on `topic` select post");
        //     tx.commit().await.expect("Failed on `topic` commit");
        //     posts
        // } else {
        //     Vec::new()
        // };
        // Some(topics::Topic {
        //     author: topics::Author {
        //         id: meta.user_id,
        //         name: meta.username,
        //         signature: meta.post_signature,
        //     },
        //     title: meta.title,
        //     posts,
        // })
    }
}
