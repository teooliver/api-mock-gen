use fake::faker;
use rand::Rng;

struct User {
    name: String,
    email: String,
}

struct Todo {
    id: i32,
    name: String,
    status: TodoItemStatus,
    user: User,
    started_at: String,
    finished_at: String,
    color: String,
}

enum TodoItemStatus {
    Done,
    InProgress,
    NotNedeed,
    ReadyToStart,
}

pub const PROJECT_COLORS: [&str; 10] = [
    "#61e294ff",
    "#7bcdbaff",
    "#9799caff",
    "#bd93d8ff",
    "#b47aeaff",
    "#d3d5d4ff",
    "#a2c5acff",
    "#9db5b2ff",
    "#878e99ff",
    "#7f6a93ff",
];


impl Todo{
    fn newRandomTodo() -> Todo {
        Todo {
           id: 100,
    name: fake::faker::lorem::en::Words(10).fake::<String>().to_string(),
    status: TodoItemStatus::Done,
    user: User {
       name: fake:faker::name::en::FirstName().fake::<String>().to_string(),
       email: fake:faker::name::en::FirstName().fake::<String>().to_string(),
    },
    started_at: "xxxxxx".to_string(),
    finished_at: "xxxxxx".to_string(),
    color: rand::thread_rng().gen_range(0..(PROJECT_COLORS.len() - 1)).to_string(),  
        }
    }
}

fn main() {
    println!("Hello, world!");
}

// fn generateJsonDb() {}

fn generateTodoListStruc(amount: u8) -> Vec<Todo> {
    let mut todos: Vec<Todo> = vec![];

    for _n in 1..amount {
        todos.push(Todo::newRandomTodo());
    }

    todos
}
