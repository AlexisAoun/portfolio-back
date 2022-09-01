use crate::models::counter::Counter;
use crate::utils::db::get_counter_collection;
use mongodb::{bson::doc, Collection};

pub async fn get_counter_value(counter_type: &String) -> u32 {
    let counter_collection: Collection<Counter> = get_counter_collection().await;
    let filter = doc! { "label": counter_type };
    let counter = match counter_collection.find_one(filter, None).await {
        Ok(counter) => counter,
        Err(err) => panic!("Error : Failed to fetch counter : {:?}", err),
    };
    counter.unwrap().value
}

pub async fn increment_counter_value(counter_type: &String) -> u32 {
    let counter_collection: Collection<Counter> = get_counter_collection().await;
    let filter = doc! { "label": counter_type };
    let mut counter_value = get_counter_value(&counter_type).await;
    counter_value += 1;
    let new_counter = Counter {
        label: counter_type.to_string(),
        value: counter_value,
    };

    let updated_counter = match counter_collection
        .find_one_and_replace(filter, new_counter, None)
        .await
    {
        Ok(updated_counter) => updated_counter,
        Err(err) => panic!(
            "Error : Failed to increment {:?} counter : {:?}",
            counter_type, err
        ),
    };

    updated_counter.unwrap().value
}
