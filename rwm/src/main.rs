use crate::connection::setup_connection;

mod connection;
mod xlib;

fn main() {
    env_logger::init();

    let root = setup_connection();
    println!("root: {:?}", root);
}
