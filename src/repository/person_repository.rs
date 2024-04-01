use std::sync::{Arc, RwLock};

use crate::person::Person;


#[derive(Clone,Default)]
pub struct PersonRepository {
    pub persons: Vec<Person>,
}


pub type PersonRepositoryState = Arc<RwLock<PersonRepository>>;