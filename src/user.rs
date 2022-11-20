use fake::Fake;
use serde::{self, Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new_random_user() -> User {
        let name = fake::faker::name::en::FirstName().fake::<String>();
        let email_provider = fake::faker::internet::en::FreeEmailProvider().fake::<String>();

        let email = format!("{}@{}", name.to_lowercase(), email_provider);

        User {
            id: Uuid::new_v4(),
            name,
            email,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}
