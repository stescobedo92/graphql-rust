use async_graphql::ID;

#[derive(Clone)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub city: String,
    pub organization: String
}