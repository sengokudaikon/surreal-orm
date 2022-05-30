use crate::configs::{get_db_from_ctx, model_cursor_to_vec, MONGO_ID_KEY};

use super::model::Post;

use async_graphql::*;
use common::error_handling::ApiHttpStatus;
use log::warn;
use mongodb::{
    bson::oid::ObjectId,
    options::{FindOneOptions, ReadConcern},
    Database,
};
use wither::{bson::doc, prelude::Model};

#[derive(Default)]
pub struct PostQueryRoot;

#[Object]
impl PostQueryRoot {
    async fn post(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the Post")] id: ObjectId,
    ) -> Result<Post> {
        let db = get_db_from_ctx(ctx)?;
        let find_one_options = FindOneOptions::builder()
            .read_concern(ReadConcern::majority())
            .build();

        Post::find_one(db, doc! {MONGO_ID_KEY: id}, find_one_options)
            .await?
            // Lazily evaluate the error:
            // Note: Always use _or_else variant of any helper function cos
            // eagerly evaluating can yield unintended consequences.
            // Readmore here  https://stackoverflow.com/questions/45547293/why-should-i-prefer-optionok-or-else-instead-of-optionok-or#:~:text=The%20only%20differences%20I%20know,Some%20data%20in%20the%20Option%20.
            .ok_or_else(|| ApiHttpStatus::NotFound("Post not found.".into()).extend())
    }

    async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        let db = get_db_from_ctx(ctx)?;
        let cursor = Post::find(db, None, None).await?;
        model_cursor_to_vec(cursor).await.map_err(|e| {
            // We don't want to expose our server internals to the end user.
            warn!("{e:?}");
            ApiHttpStatus::BadRequest("Could not fetch posts. Try again later".into()).extend()
        })
    }
}
