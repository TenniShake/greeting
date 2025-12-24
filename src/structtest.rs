#[derive(Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}

pub enum IpAddrKind {
    V4,
    V6,
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub enum Book {
    Paperty(u32),
    Electronic(String)
}

