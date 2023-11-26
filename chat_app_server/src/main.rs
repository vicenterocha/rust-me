#[macro_use] extern crate rocket;
use dashmap::DashMap;
use rocket::State;
use rocket::{Build, Rocket};

type UserMessages = State<DashMap<String, Vec<String>>>;

#[get("/user_messages/<username>")]
fn user_messages(username: &str, user_messages: &UserMessages) -> String {
    if let Some(messages) = user_messages.get(username) {
        messages.join("; ")
    } else {
        format!("Username {} not found", username)
    }
}

// Your Rocket handler
#[post("/update_data")]
fn update_data(user_messages: &UserMessages) {
    // Update the DashMap without acquiring a lock
    let a = "Here is a sentence.Here is another one".split(".").map(String::from).collect();
    user_messages.insert("gary".to_string(), a);
}

#[launch]
fn rocket() -> Rocket<Build> {
    let initial_data: DashMap<String, Vec<String>> = DashMap::new();

    rocket::build()
        .mount("/", routes![update_data, user_messages])
        .manage(initial_data)
}


#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn user_messages_should_return_not_found_response_when_no_messages() {
        // Given
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        // When
        let username = "nonexistent_user";
        let response = client.get(format!("/user_messages/{}", username)).dispatch();

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Username nonexistent_user not found");
    }

    #[test]
    fn user_messages_should_return_user_messages_when_messages_exist() {
        // Given
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        client.post("/update_data").dispatch();

        // When
        let username = "gary";
        let response = client.get(format!("/user_messages/{}", username)).dispatch();

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Here is a sentence; Here is another one");
    }
}