use crate::model::subscriber::Subscriber;
use dashmap::DashMap;
use lazy_static::lazy_static;

// Singleton of Database
lazy_static! {
    static ref SUBSCRIBERS: DashMap<String, DashMap<String, Subscriber>> = DashMap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository {
    pub fn add(product_type: &str, subscriber: Subscriber) -> Subscriber {
        let subscribe_value = subscriber.clone();
        if SUBSCRIBERS.get(product_type).is_none() {
            SUBSCRIBERS.insert(String::from(product_type), DashMap::new());
        };

        SUBSCRIBERS
            .get(product_type)
            .unwrap()
            .insert(subscribe_value.url.clone(), subscribe_value);
        return subscriber;
    }
}
