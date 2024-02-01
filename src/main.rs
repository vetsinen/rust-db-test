#[tokio::main]
async fn main() {
    println!("Hello, world!");
    //let _db = libsql_client::Client::from_env().await.unwrap();
    let conn = sqlite::open("file:////home/webdev/test.db").unwrap();

    //     CREATE TABLE users (name TEXT, age INTEGER);
    let query = "
    INSERT INTO users VALUES ('Maggie', 42);
    INSERT INTO users VALUES ('Bill', 69);
    ";
    conn.execute(query).unwrap();
}
