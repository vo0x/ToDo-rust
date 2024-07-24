use std::{env, usize};
use std::fs::OpenOptions;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [arguments]", args[0]);
        return Ok(());
    }

    let command = &args[1];
    let task_args = if args.len() > 2 {
        Some(args[2..].join(" "))
    } else {
        None
    };

    let file_path = String::from("tasks.txt");
    let todo = ToDo::new(&file_path);

    match command.as_str() {
        "-a" => {
            if let Some(task) = task_args {
                todo.add_task(task)?;
            } else {
                eprintln!("Error: No task provided for -a command");
            }
        }
        "-r" => {
            todo.review()?;
        }
        
        "-d" => if let Some(index_str) = task_args{
            if let Ok(index) = index_str.parse::<usize>(){
                todo.delete(index)?} else{
                    eprintln!("Err")
                }
        } else {
            eprintln!("meow")
        }
        _ => {
            eprintln!("Error: Unknown command");
        }
    }

    Ok(())
}

struct ToDo {
    file_path: String,
}

impl ToDo {
    fn new(file_path: &str) -> ToDo {
        ToDo {
            file_path: file_path.to_string(),
        }
    }

    fn add_task(&self, task: String) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true) // Use append to add tasks without overwriting
            .open(&self.file_path)?;

        writeln!(file, "{}", task)?; // Use writeln! to add a newline
        println!("Task '{}' has been successfully added", task);
        Ok(())
    }

    fn review(&self) -> io::Result<()> {
        let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let tasks: Vec<String> = content.lines().map(|line| line.to_string()).collect();

        if tasks.is_empty() {
            println!("No tasks :/");
        } else {
            for (i, task) in tasks.iter().enumerate() {
                println!("{} - {}", i + 1, task);
            }
        }
        Ok(())
    }
    fn delete(&self, index : usize) -> io::Result<()>{
        let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let mut tasks : Vec<String> = content.lines().map(|line | line.to_string()).collect();

        if index == 0 || index > tasks.len() {
            eprintln!("Error: Invalid index");
            return Ok(());
        }

        tasks.remove(index - 1);

        let mut file = OpenOptions::new().write(true).truncate(true).open(&self.file_path)?;
        for task in tasks {
            writeln!(file, "{}", task)?;
            
        }
        println!("task {}  done ", index);
    
        Ok(())

    }
}
