use std::collections::HashMap;

use firebase_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

#[tokio::main]
async fn main(){
    let user = User {
        name: "Shivam Modi".to_string(),
        age: 21,
        email: "shivam27modi@gmail.com".to_string(),
    };


    let firebase = Firebase::new("https://rustcurd-default-rtdb.europe-west1.firebasedatabase.app/").unwrap();

    let response = set_user(&firebase, &user).await;

    let mut user = get_user(&firebase, &response.name).await;
    println!("{:?}", user);

    let users = get_users(&firebase).await;

    println!("users are {:?}", users);

    user.email = "lastupdate.mail@gmail.com".to_string();

    let updated_user = update_user(&firebase, &response.name, &user).await;

    println!("updated user is {:?}", updated_user);

    delete_user(&firebase, &response.name).await;

    println!("user deleted");
}

async fn set_user(firebase_client: &Firebase, user: &User) -> Response{
    let firebase = firebase_client.at("users");
    let _users = firebase.set::<User>(&user).await;
    return string_to_response(&_users.unwrap().data);
}

async fn get_users(firebase_client: &Firebase) -> HashMap<String, User>{
    let firebase = firebase_client.at("users");
    let users = firebase.get::<HashMap<String, User>>().await;
    return users.unwrap();
}


async fn get_user(firebase_client: &Firebase, id: &String) -> User{
    let firebase = firebase_client.at("users").at(&id);
    let user = firebase.get::<User>().await;
    return user.unwrap();
}

async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> User{
    let firebase = firebase_client.at("users").at(&id);
    let _user = firebase.update::<User>(&user).await;
    return string_to_user(&_user.unwrap().data);
}

async fn delete_user(firebase_client: &Firebase, id: &String){
    let firebase = firebase_client.at("users").at(&id);
    let _result = firebase.delete().await;
}

// convert string to response
fn string_to_response(s: &str) -> Response{
    let response: Response = serde_json::from_str(s).unwrap();
    response
}

// convert string to user
fn string_to_user(s: &str) -> User{
    let user: User = serde_json::from_str(s).unwrap();
    user
}
