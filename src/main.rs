use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path::end().map(|| "Hello, World!");
    warp::serve(hello).run(([0, 0, 0, 0], 3000)).await;
}
