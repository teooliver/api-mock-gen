use crate::{comment::Comment, post::Post, task::Task, user::User};
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppData {
    pub tasks: Vec<Task>,
    pub users: Vec<User>,
    pub posts: Vec<Post>,
    pub comments: Vec<Comment>,
}

impl AppData {
    // ===========================
    // User "collection" functions
    // ===========================

    pub fn get_users(&self) -> &Vec<User> {
        &self.users
    }

    pub fn get_user_by_id(&self, id: &Uuid) -> Option<&User> {
        let user = self.users.iter().find(|user| user.id == *id);
        user
    }

    pub fn create_user(&mut self, new_user: User) {
        self.users.push(new_user)
    }

    pub fn remove_user_by_id(&mut self, id: &Uuid) -> Option<User> {
        // TODO: Delete all tasks, posts and comments from this user
        let index = self.users.iter().position(|user| user.id == *id);

        match index {
            Some(index) => {
                let removed_user = self.users.remove(index);
                return Some(removed_user);
            }
            None => return None,
        };
    }

    // ============================
    // Tasks "collection" functions
    // ============================

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn get_tasks_by_id(&self, id: &Uuid) -> Option<&Task> {
        let task = self.tasks.iter().find(|task| task.id == *id);
        task
    }

    pub fn remove_task_by_id(&mut self, id: &Uuid) -> Option<Task> {
        let index = self.tasks.iter().position(|task| task.id == *id);

        match index {
            Some(index) => {
                let removed_task = self.tasks.remove(index);
                return Some(removed_task);
            }
            None => return None,
        };
    }

    pub fn get_all_user_tasks(&self, user_id: &Uuid) -> Option<Vec<Task>> {
        // let task = self.tasks.iter().find(|task| task.id == *id).unwrap();
        let mut tasks: Vec<Task> = vec![];
        self.tasks.iter().for_each(|task| {
            if task.user.id == *user_id {
                tasks.push(task.clone())
            }
        });

        if tasks.len() == 0 {
            return None;
        }

        Some(tasks.clone())
    }

    // ============================
    // Posts "collection" functions
    // ============================

    fn get_posts(&self) -> &Vec<Task> {
        todo!();
    }

    fn get_posts_by_id(&self, id: &Uuid) -> &Task {
        todo!();
    }

    fn get_all_user_posts(&self, id: &Uuid) -> Option<&Vec<Task>> {
        todo!()
    }

    fn remove_post_by_id(&self, id: &Uuid) -> Option<&Vec<Task>> {
        todo!()
    }

    // ===============================
    // Comments "collection" functions
    // ===============================

    fn get_comments(&self) -> &Vec<Task> {
        todo!();
    }

    fn get_comments_by_id(&self, id: &Uuid) -> &Task {
        todo!();
    }

    fn get_all_user_comments(&self, id: &Uuid) -> Option<&Vec<Task>> {
        todo!()
    }

    fn remove_comment_by_id(&self, id: &Uuid) -> Option<&Vec<Task>> {
        todo!()
    }
}

// Should this be a method of AppData instead? (So we can regenerate data at any time by calling an endpoint)
// This way we can play with the data, delete, add and etc to the db, and then just call a endpoint to regenerate and start from scratch
// TODO: add amount of posts and amount_of_comments as "optional" params.
pub fn generate_app_data(amount_of_tasks: u8, amount_of_users: u8) -> AppData {
    let mut users: Vec<User> = vec![];
    for _n in 1..=amount_of_users {
        users.push(User::new_random_user());
    }

    let mut tasks: Vec<Task> = vec![];
    for _n in 1..=amount_of_tasks {
        tasks.push(Task::new_random_task(&Some(
            users[rand::thread_rng().gen_range(0..users.len())].clone(),
        )));
    }

    let mut posts: Vec<Post> = vec![];
    for _n in 1..=20 {
        posts.push(Post::new_random_post(&Some(
            users[rand::thread_rng().gen_range(0..users.len())].clone(),
        )));
    }

    let mut comments: Vec<Comment> = vec![];
    for post in &posts {
        for _n in 1..5 {
            let user = &Some(users[rand::thread_rng().gen_range(0..users.len())].clone());
            comments.push(Comment::new_random_comment(user, &post));
        }
    }

    AppData {
        tasks,
        users,
        posts,
        comments,
    }
}
