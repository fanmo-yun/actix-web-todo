use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct User {
    pub username: String,
    pub passwd: String
}

#[derive(Deserialize, Debug)]
pub struct EnrollUser {
    pub eusername: String,
    pub epasswd: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ToDo {
    pub todo_title: String,
    pub date_time: String
}

#[derive(Deserialize, Debug)]
pub struct NewToDo {
    pub todo_title: String,
    pub date_time: String
}
