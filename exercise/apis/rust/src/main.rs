mod models;
mod handlers;
mod routes;

use crate::routes::routes;
use std::sync::atomic::AtomicUsize;

pub static VISITS: AtomicUsize = AtomicUsize::new(0);

#[tokio::main]
async fn main() {
    let routes = routes();
    println!("Server started at http:://localhost:8000");
    warp::serve(routes).run(([127 ,0 ,0 ,1 ], 8000)).await;
}
