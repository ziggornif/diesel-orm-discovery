use std::env::args;

use diesel_demo::{db_connect, delete_post};

fn main() {
    let post_id = args()
        .nth(1)
        .expect("get_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut db_connect();

    delete_post(connection, post_id);
}
