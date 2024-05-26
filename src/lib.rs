// copyright
// blablabla

//! # Diesel demo module
//!
//! This module is a diesel features test

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::Post;
use std::env;

use crate::models::NewPost;

pub mod models;
pub mod schema;

/// Create db connection
///
/// ```
/// use diesel::pg::PgConnection;
/// use diesel_demo::*;
/// let conn = &mut db_connect();
/// ```
pub fn db_connect() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

/// Create a new post
///
/// **Note : posts are not published during creation. You must use the [`publish_post`] method for this.**
///
/// ```
/// use diesel::pg::PgConnection;
/// use diesel_demo::*;
///
/// let conn = &mut db_connect();
/// let created = create_post(conn, "foo", "bar");
/// ```
pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };
    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

/// Publish a post
///
/// **Note : the post must be created before with [`create_post`].**
///
/// ```
/// use diesel::pg::PgConnection;
/// use diesel_demo::*;
///
/// let conn = &mut db_connect();
/// let created = publish_post(conn, 1);
/// ```
pub fn publish_post(conn: &mut PgConnection, post_id: i32) -> Post {
    use self::schema::posts::dsl::{posts, published};

    diesel::update(posts.find(post_id))
        .set(published.eq(true))
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error updating post")
}

/// List published posts
///
/// ```
/// use diesel::pg::PgConnection;
/// use diesel_demo::*;
///
/// let conn = &mut db_connect();
/// let posts = list_posts(conn);
/// for post in posts {
///     println!("{}", post.title);
///     println!("-----------\n");
///     println!("{}", post.body);
/// }
/// ```
pub fn list_posts(conn: &mut PgConnection) -> Vec<Post> {
    use self::schema::posts::dsl::*;

    posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(conn)
        .expect("Error loading posts")
}

/// Get a post by id
///
/// ```
/// use diesel::pg::PgConnection;
/// use diesel_demo::*;
///
/// let conn = &mut db_connect();
/// let post_id = 1;
/// match get_post(conn, post_id) {
///     Some(post) => print_post!(post.id, post.title),
///     None => println!("Unable to find post {}", post_id)
/// }
/// ```
pub fn get_post(conn: &mut PgConnection, post_id: i32) -> Option<Post> {
    use self::schema::posts::dsl::*;

    posts
        .find(post_id)
        .select(Post::as_select())
        .first(conn)
        .optional()
        .expect("Error getting post")
}

/// Delete a post by id
///
/// ```
/// use diesel::pg::PgConnection;
/// use diesel_demo::*;
///
/// let conn = &mut db_connect();
/// delete_post(conn, 1);
/// ```
pub fn delete_post(conn: &mut PgConnection, post_id: i32) {
    use self::schema::posts::dsl::*;

    let num_deleted = diesel::delete(posts.find(post_id))
        .execute(conn)
        .expect("Error deleting post");

    println!("Deleted {}", num_deleted);
}

/// Print post macro
///
/// ```
/// use diesel_demo::*;
///
/// print_post!("foo", "bar");
/// ```
#[macro_export]
macro_rules! print_post {
    ($id:expr, $title:expr) => {
        println!("Post with id: {} has a title: {}", $id, $title)
    };
}
