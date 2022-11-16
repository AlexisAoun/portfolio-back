use crate::models::tag::Tag;
use crate::utils::db;
use mongodb::Collection;
use futures::stream::TryStreamExt;

pub async fn get_all_tags() -> Vec<Tag> {
    let col: Collection<Tag> = db::get_tag_collection().await;
    let cursor = col.find(None,None).await.unwrap();
    cursor.try_collect().await.unwrap()
}

