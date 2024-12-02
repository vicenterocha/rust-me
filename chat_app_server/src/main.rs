#[macro_use]
extern crate rocket;
use dashmap::DashMap;
use rocket::futures::{SinkExt, StreamExt};
use rocket::State;
use rocket::{Build, Rocket};
use rocket::serde::{Deserialize, json::Json};


type UserMessages = State<DashMap<String, Vec<String>>>;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message<'r> {
    body: &'r str,
}


#[get("/user_messages/<username>")]
fn user_messages(username: &str, user_messages: &UserMessages) -> String {
    if let Some(messages) = user_messages.get(username) {
        messages.join("; ")
    } else {
        format!("Username {} not found", username)
    }
}

#[post("/update_data/<username>", data = "<message>")]
fn update_data(username: &str, message: Json<Message<'_>>, user_messages: &UserMessages) {
    // TODO: for now this is always creating a new list - support append to list of messages
    user_messages.insert(username.to_string(), vec![message.body.to_string()]);
}

#[get("/hello/<name>")]
fn hello(ws: ws::WebSocket, name: &str) -> ws::Channel<'_> {
    ws.channel(move |mut stream| {
        Box::pin(async move {
            let message = format!("Hello, {}!", name);
            let _ = stream.send(message.into()).await;
            Ok(())
        })
    })
}

#[get("/echo")]
fn echo(ws: ws::WebSocket) -> ws::Channel<'static> {
    ws.channel(move |mut stream| {
        Box::pin(async move {
            while let Some(message) = stream.next().await {
                let _ = stream.send(message?).await;
            }

            Ok(())
        })
    })
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
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::http::ContentType;

    #[test]
    fn user_messages_should_return_not_found_response_when_no_messages() {
        // Given
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        // When
        let username = "nonexistent_user";
        let response = client
            .get(format!("/user_messages/{}", username))
            .dispatch();

        // Then
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap(),
            "Username nonexistent_user not found"
        );
    }

    #[test]
    fn user_messages_should_return_allow_to_add_messages_to_a_username() {
        // Given
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let username = "quinn";

        // pre-when-assert to make sure username has no messages
        let response = client
            .get(format!("/user_messages/{}", username))
            .dispatch();
        assert_eq!(
            response.into_string().unwrap(),
            format!("Username {} not found", username)
        );

        // When
        let response = client
            .post(format!("/update_data/{}", username))
            .header(ContentType::JSON)
            .body(
                r##"{
                    "body": "Everybody hates KVN"
                }"##,
            )
            .dispatch();

        // Then
        let response = client
            .get(format!("/user_messages/{}", username))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Everybody hates KVN");
    }

}
