use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Person {
    id: u64,
    pub name: String,
    pub age: u8,
    pub email: String,
}

impl Person {
    pub fn new(name: String, age: u8, email: String) -> Self {
        Person {
            id: 0x01,
            name,
            age,
            email,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PersonDto {
    pub name: String,
    pub age: u8,
    pub email: String,
}
