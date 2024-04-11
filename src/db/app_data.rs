use std::vec;

use rand::Rng;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::models::Task;
use crate::models::User;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppData {
    pub tasks: Vec<Task>,
    pub users: Vec<User>,
}

impl AppData {
    pub fn change_app_state(&mut self) {
        let new_state = Self::generate_app_data(100, 5);

        self.users = new_state.users;
        self.tasks = new_state.tasks;
    }

    // TODO: add amount of posts and amount_of_comments as "optional" params.
    pub fn generate_app_data(amount_of_tasks: u8, amount_of_users: u8) -> Self {
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

        AppData { tasks, users }
    }

    // ===========================
    // User "collection" functions
    // ===========================

    pub fn drop_users(&mut self) -> &Vec<User> {
        self.users = vec![];
        &self.users
    }

    pub fn get_users(&self) -> &Vec<User> {
        &self.users
    }

    pub fn get_user_by_id(&self, id: &Uuid) -> Option<&User> {
        let user = self.users.iter().find(|user| user.id == *id);
        user
    }

    pub fn create_user(&mut self, new_user: User) -> User {
        // Ensure email is unique?
        self.users.push(new_user.clone());
        new_user
    }

    pub fn remove_user_by_id(&mut self, id: &Uuid) -> Option<User> {
        // TODO: Delete all tasks, posts and comments from this user (or anonymize)
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

    pub fn drop_tasks(&mut self) -> &Vec<Task> {
        self.tasks = vec![];
        &self.tasks
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn get_tasks_by_id(&self, id: &Uuid) -> Option<&Task> {
        let task = self.tasks.iter().find(|task| task.id == *id);
        task
    }

    // TODO: Return DbResult instead of Option and return error
    // if task is not found using db::error
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
        let mut tasks: Vec<Task> = vec![];
        self.tasks.iter().for_each(|task| {
            if task.user_ref == *user_id {
                tasks.push(task.clone())
            }
        });

        if tasks.len() == 0 {
            return None;
        }

        Some(tasks.clone())
    }

    // TODO: Return Result<Task> instead
    // TODO: Should we recieve the NewTask props and build the task here
    // instead of in the router handler?
    pub fn create_task(&mut self, new_task: Task) -> Task {
        self.tasks.push(new_task.clone());
        new_task
    }

    // TODO: Return Result<Task> instead as Task could not be found
    // in that case throw an Error
    pub fn update_task(&mut self, new_task: Task) -> Task {
        let id = new_task.id.clone();

        // find task by id and mutate it for the new new one
        for task in self.tasks.iter_mut() {
            if task.id == id {
                *task = new_task.clone();
            }
        }

        new_task
    }
}
