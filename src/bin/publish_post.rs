use diesel_demo::*;
use std::env::args;

fn main() {
    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut db_connect();

    let post = publish_post(connection, id);

    println!("Published post {}", post.title);
}
