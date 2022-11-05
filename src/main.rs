mod comment;
mod helpers;
mod post;
mod task;
mod user;

use std::fs::File;
use std::io::BufWriter;

use comment::Comment;
use post::Post;
use rand::Rng;
use serde::{self, Serialize};
use task::Task;
use user::User;

fn main() {
    let AppData {
        todos,
        users,
        posts,
        ..
    } = generate_todo_list_struc(10, 4);
    generate_json_db(&todos, "tasks_json_db.json".to_string());
    generate_json_db(&users, "users_json_db.json".to_string());
    generate_json_db(&posts, "posts_json_db.json".to_string());
}

struct AppData {
    todos: Vec<Task>,
    users: Vec<User>,
    posts: Vec<Post>,
    comments: Vec<Comment>,
}

fn generate_json_db<T: Serialize>(input: &Vec<T>, output_path: String) {
    let mut writer = BufWriter::new(File::create(output_path).unwrap());
    serde_json::to_writer_pretty(&mut writer, &input).unwrap();
}

// TODO: argument should be optional
fn generate_todo_list_struc(amount_of_tasks: u8, amount_of_users: u8) -> AppData {
    let mut comments: Vec<Comment> = vec![];

    let mut users: Vec<User> = vec![];
    for _n in 1..=amount_of_users {
        users.push(User::create_random_user());
    }

    let mut todos: Vec<Task> = vec![];
    for _n in 1..=amount_of_tasks {
        todos.push(Task::new_random_task(&Some(
            users[rand::thread_rng().gen_range(0..users.len())].clone(),
        )));
    }

    let mut posts: Vec<Post> = vec![];
    for _n in 1..=20 {
        posts.push(Post::new_random_post(&Some(
            users[rand::thread_rng().gen_range(0..users.len())].clone(),
        )));
    }

    AppData {
        todos,
        users,
        posts,
        comments,
    }
}
