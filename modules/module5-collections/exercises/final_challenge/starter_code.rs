use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::fmt;

#[derive(Debug)]
enum TaskStatus {
    Pending,
    Completed,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::Completed => write!(f, "Completed"),
        }
    }
}

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    description: String,
    due_date: Option<String>,
    status: TaskStatus,
}

impl Task {
    fn display(&self) {
        println!(
            "[{}] {}: {} (Due: {}) - {}",
            self.id,
            self.title,
            self.description,
            self.due_date.as_deref().unwrap_or("None"),
            self.status
        );
    }
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String, description: String, due_date: Option<String>) -> &Task {
        let task = Task {
            id: self.next_id,
            title,
            description,
            due_date,
            status: TaskStatus::Pending,
        };
        self.next_id += 1;
        self.tasks.push(task);
        self.tasks.last().unwrap()
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found.");
        } else {
            for task in &self.tasks {
                task.display();
            }
        }
    }

    fn complete_task(&mut self, id: u32) -> bool {
        for task in &mut self.tasks {
            if task.id == id {
                task.status = TaskStatus::Completed;
                return true;
            }
        }
        false
    }

    fn delete_task(&mut self, id: u32) -> bool {
        let len = self.tasks.len();
        self.tasks.retain(|task| task.id != id);
        len != self.tasks.len()
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        for task in &self.tasks {
            writeln!(
                file,
                "{},{},{},{},{}",
                task.id,
                task.title.replace(",", " "),
                task.description.replace(",", " "),
                task.due_date.as_deref().unwrap_or(""),
                match task.status {
                    TaskStatus::Pending => "Pending",
                    TaskStatus::Completed => "Completed",
                }
            )?;
        }
        Ok(())
    }

    fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
        let file = OpenOptions::new().read(true).open(filename)?;
        let reader = BufReader::new(file);
        self.tasks.clear();
        self.next_id = 1;
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.splitn(5, ',').collect();
            if parts.len() == 5 {
                let id = parts[0].parse().unwrap_or(self.next_id);
                let title = parts[1].to_string();
                let description = parts[2].to_string();
                let due_date = if parts[3].is_empty() { None } else { Some(parts[3].to_string()) };
                let status = match parts[4] {
                    "Completed" => TaskStatus::Completed,
                    _ => TaskStatus::Pending,
                };
                self.tasks.push(Task {
                    id,
                    title,
                    description,
                    due_date,
                    status,
                });
                self.next_id = id + 1;
            }
        }
        Ok(())
    }

    fn filter_by_status(&self, status: TaskStatus) {
        for task in &self.tasks {
            if std::mem::discriminant(&task.status) == std::mem::discriminant(&status) {
                task.display();
            }
        }
    }

    fn filter_by_due_date(&self, due: &str) {
        for task in &self.tasks {
            if let Some(ref d) = task.due_date {
                if d == due {
                    task.display();
                }
            }
        }
    }
}

enum Command {
    Add { title: String, description: String, due_date: Option<String> },
    List,
    Complete { id: u32 },
    Delete { id: u32 },
    Save { filename: String },
    Load { filename: String },
    FilterStatus { status: TaskStatus },
    FilterDue { due: String },
    Quit,
    Unknown,
}

fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
    match parts[0].to_lowercase().as_str() {
        "add" => {
            let args: Vec<&str> = parts.get(1).unwrap_or(&"").splitn(3, ';').collect();
            let title = args.get(0).unwrap_or(&"").trim().to_string();
            let description = args.get(1).unwrap_or(&"").trim().to_string();
            let due_date = args.get(2).map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
            Command::Add { title, description, due_date }
        }
        "list" => Command::List,
        "complete" => {
            let id = parts.get(1).unwrap_or(&"0").trim().parse().unwrap_or(0);
            Command::Complete { id }
        }
        "delete" => {
            let id = parts.get(1).unwrap_or(&"0").trim().parse().unwrap_or(0);
            Command::Delete { id }
        }
        "save" => {
            let filename = parts.get(1).unwrap_or(&"tasks.csv").trim().to_string();
            Command::Save { filename }
        }
        "load" => {
            let filename = parts.get(1).unwrap_or(&"tasks.csv").trim().to_string();
            Command::Load { filename }
        }
        "filter_status" => {
            let status = match parts.get(1).unwrap_or(&"pending").trim().to_lowercase().as_str() {
                "completed" => TaskStatus::Completed,
                _ => TaskStatus::Pending,
            };
            Command::FilterStatus { status }
        }
        "filter_due" => {
            let due = parts.get(1).unwrap_or(&"").trim().to_string();
            Command::FilterDue { due }
        }
        "quit" => Command::Quit,
        _ => Command::Unknown,
    }
}

fn main() {
    let mut task_manager = TaskManager::new();
    let stdin = io::stdin();

    println!("Rust Task Manager");
    println!("Commands:");
    println!("  add <title>;<description>;<due_date>");
    println!("  list");
    println!("  complete <id>");
    println!("  delete <id>");
    println!("  save <filename>");
    println!("  load <filename>");
    println!("  filter_status <pending|completed>");
    println!("  filter_due <due_date>");
    println!("  quit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            println!("Failed to read input.");
            continue;
        }
        match parse_command(&input) {
            Command::Add { title, description, due_date } => {
                if title.is_empty() {
                    println!("Title cannot be empty.");
                } else {
                    let task = task_manager.add_task(title, description, due_date);
                    println!("Added task:");
                    task.display();
                }
            }
            Command::List => task_manager.list_tasks(),
            Command::Complete { id } => {
                if task_manager.complete_task(id) {
                    println!("Task {} marked as completed.", id);
                } else {
                    println!("Task not found.");
                }
            }
            Command::Delete { id } => {
                if task_manager.delete_task(id) {
                    println!("Task {} deleted.", id);
                } else {
                    println!("Task not found.");
                }
            }
            Command::Save { filename } => {
                match task_manager.save_to_file(&filename) {
                    Ok(_) => println!("Tasks saved to {}", filename),
                    Err(e) => println!("Failed to save: {}", e),
                }
            }
            Command::Load { filename } => {
                match task_manager.load_from_file(&filename) {
                    Ok(_) => println!("Tasks loaded from {}", filename),
                    Err(e) => println!("Failed to load: {}", e),
                }
            }
            Command::FilterStatus { status } => task_manager.filter_by_status(status),
            Command::FilterDue { due } => task_manager.filter_by_due_date(&due),
            Command::Quit => {
                println!("Goodbye!");
                break;
            }
            Command::Unknown => println!("Unknown command."),
        }
    }
}