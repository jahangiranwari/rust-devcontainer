use warp::Filter;

#[tokio::main]
async fn main() {
    let message = "Application is booting up.. 🚀";
    println!("{}", message);

    let hello = warp::path::end().map(|| "Hello, World!");
    warp::serve(hello).run(([0, 0, 0, 0], 3000)).await;
}
