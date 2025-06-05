use serde::Deserialize;

#[derive(Deserialize)]
struct UserQuery {
    name: String,
}
