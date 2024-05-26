use diesel_demo::*;

fn main() {
    let connection = &mut db_connect();
    let results = list_posts(connection);

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
