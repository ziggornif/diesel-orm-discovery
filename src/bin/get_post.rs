use std::env::args;

use diesel_demo::{db_connect, get_post, print_post};

fn main() {
    let post_id = args()
        .nth(1)
        .expect("get_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut db_connect();

    match get_post(connection, post_id) {
        Some(post) => print_post!(post.id, post.title),
        None => println!("Unable to find post {}", post_id),
    }
}
