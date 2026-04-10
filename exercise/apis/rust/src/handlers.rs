use std::sync::atomic::Ordering;

use super::models::Post;
use crate::VISITS;

pub async fn get_post() -> Result<impl warp::Reply, warp::Rejection> {
    let visits = VISITS.fetch_add(1, Ordering::Relaxed) + 1;
    let post = Post {
        title: String::from("Ich bin Tommy"),
        body: String::from("Meeeooooowwwww"),

    };

    let specialPost = Post {
        title: String::from("Ich bin Tommy"),
        body: String::from("Hier mein Geschenk: Dieses lustige Toter Vogel Ding"),
    };
    if visits % 2 == 0 {
        Ok(warp::reply::json(&post))
    } else {
        Ok(warp::reply::json(&specialPost))
    }
}
