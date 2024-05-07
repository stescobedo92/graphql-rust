use async_graphql::ID;
use crate::user_service::User;
pub struct DB;

impl DB {
    pub fn get_data(&self) -> Vec<User> {
        vec![
            User {
                id: ID::from("1"),
                name:String::from("John Doe"),
                email:String::from("johndoe@example.com"),
                phone:String::from("111-222-3333"),
                address:String::from("123 Main St"),
                city:String::from("New York"),
                organization:String::from("Organization Test-1")
            },
            User {
                id: ID::from("2"),
                name:String::from("John Doe 2"),
                email:String::from("johndoe-2@example.com"),
                phone:String::from("222-333-4444"),
                address:String::from("111 Main St"),
                city:String::from("New York 2"),
                organization:String::from("Organization Test-2")
            }
        ]
    }
}