use inquire ::{Select,Text,Confirm};
use serde :: {Serialize,Deserialize};
use std :: fs;


#[derive(Serialize,Deserialize,Clone,Debug)]
struct Task{
    id : usize,
    title : String,
    completed : bool,
}


const FILE_PATH : &str = "C:\\Users\\acer\\todo_interactive\\tasks.json";

fn main() {
    let mut tasks = load_tasks();

    loop {
        let options = vec![
            "Add Task",
            "View Tasks",
            "Edit Task",
            "Unmark",
            "Mark Task as Done",
            "Delete Task",
            "Exit",
        ];

        let choice =Select::new("What would you like to do?", options).prompt().unwrap();

        match choice {
            "Add Task" => add_task(&mut tasks),
            "View Tasks" => view_tasks(&tasks),
            "Edit Task" => edit_task(&mut tasks),
            "Mark Task as Done" => mark_done(&mut tasks),
            "Unmark" => mark_undone(&mut tasks),
            "Delete Task" => delete_task(&mut tasks),
            "Exit" => {
                save_tasks(&tasks);
                println!("Goodbye! 👋");
                break;
            }
            _ => {}
        }
    }
}

fn add_task(tasks : &mut Vec<Task>){

    let title = Text::new("Enter task title:")
        .prompt()
        .unwrap();

    let id = tasks.len() + 1;
    tasks.push(Task {
        id,
        title,
        completed: false,
    });
    println!("✅ Task added!");
    save_tasks(tasks);
}

fn view_tasks(tasks : &[Task]){
    if tasks.is_empty() {
        println!("📭 No tasks yet!");
        return;
    }
    println!("\n📋 Your Tasks:");
    for task in tasks {
        let status = if task.completed { "✓" } else { "✗" };
        println!("[{}] {} - {}", task.id, status, task.title);
    }
    println!();
}

fn edit_task(tasks : &mut Vec<Task>,){
    if tasks.is_empty() {
        println!("📭 No tasks to Edit!");
        return;
    }
    let task_titles: Vec<String> = tasks.iter().map(|t| format!("{} - {}", t.id, t.title)).collect();

    let selected = Select::new("Select the Task to Edit",task_titles).prompt().unwrap();

    let task_id: usize = selected.split(" - ").next().unwrap().parse().unwrap();

    if let Some(task) = tasks.iter_mut().find(|t| t.id ==task_id){
        println!("Current title: {}", task.title);

    let new_title = Text::new ("Enter new title:").with_default(&task.title)
    .prompt().unwrap();
     task.title = new_title;
     println!("✏️ Task updated successfully!");
        save_tasks(tasks);
    }


}

fn mark_undone(tasks : &mut Vec<Task>){
    if tasks.is_empty() {
        println!("📭 No tasks to mark as done!");
        return;
    }

    let task_titles: Vec<String> = tasks 
        .iter()
        .map(|t| format!("{} - {}", t.id, t.title))
        .collect();
    
    let selected : String = Select::new("Select task to mark as done:", task_titles)
    .prompt().unwrap();

    let task_id: usize = selected.split(" - ").next().unwrap().parse().unwrap();

    if let Some(task  ) = tasks.iter_mut().find(|t| t.id == task_id){
        task.completed = false;
        println!("✅ Task marked as undone!");
        save_tasks(tasks);

    }

}

fn mark_done(tasks : &mut Vec<Task>){
    if tasks.is_empty() {
        println!("📭 No tasks to mark as done!");
        return;
    }

    let task_titles: Vec<String> = tasks
        .iter()
        .map(|t| format!("{} - {}", t.id, t.title))
        .collect();

    let selected = Select::new("Select task to mark as done:", task_titles)
        .prompt()
        .unwrap();

    let task_id: usize = selected.split(" - ").next().unwrap().parse().unwrap();

    if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
        task.completed = true;
        println!("✅ Task marked as done!");
        save_tasks(tasks);
    }


}

fn delete_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("📭 No tasks to delete!");
        return;
    }

    let task_titles: Vec<String> = tasks
        .iter()
        .map(|t| format!("{} - {}", t.id, t.title))
        .collect();

    let selected = Select::new("Select task to delete:", task_titles)
        .prompt()
        .unwrap();

    let task_id: usize = selected.split(" - ").next().unwrap().parse().unwrap();

    let confirm = Confirm::new("Are you sure you want to delete this task?")
        .with_default(false)
        .prompt()
        .unwrap();

    if confirm {
        tasks.retain(|t| t.id != task_id);
        println!("🗑️ Task deleted!");
        save_tasks(tasks);
    }
}



fn load_tasks() -> Vec<Task> {
    if let Ok(data) = fs::read_to_string(FILE_PATH) {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn save_tasks(tasks: &[Task]) {
    let json = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(FILE_PATH, json).unwrap();
}
