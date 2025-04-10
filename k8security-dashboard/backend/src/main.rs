use backend::establish_connection;

mod schema;
mod models;

fn main() {
    println!("Hello, world!");
    let connection = establish_connection();
}
