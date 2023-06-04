/*
Project: A Simple Command Line Todo List
done
*/


#[allow(dead_code)]
fn main() {
    use std::io::{self, Write};
    
    let mut lists: Vec<List> = vec![];
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    
    println!("\nWelcome to the Command Line To Do List!");
    println!("\nType help to show commands");
    
    loop {
        print!("> ");
        stdout.flush().unwrap();
        
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        let parts: Vec<&str> = buffer.split_whitespace().collect();
        
        if parts.is_empty(){
            continue;
        }
        
        match parts[0] {
            "create" => {
                if parts.len() != 3 || parts[1] != "list" {
                    println!("Usage: create list <name>\n");
                    continue;
                }
                lists.push(List::new(parts[2].to_string()));
                println!("List created!\n");
                continue;
            }
            
            "delete" => {
                match parts[1] {
                    "list" => {
                        if parts.len() != 3 {
                            println!("Usage: delete list <name>\n");
                            continue;
                        }
                        
                        match lists.iter().position(|list| list.name == parts[2]) {
                            Some(i) => {
                                lists.remove(i);
                                println!("List deleted succesfully\n");
                            }
                            None => {
                                println!("List not found\n");
                            }
                        }
                        continue;
                    }
                    
                    "task" => {
                        if parts.len() != 4 {
                            println!("Usage: delete task <list_name> <task_name>\n");
                            continue;
                        }
                        let mut found = false;
                        for list in &mut lists {
                            if parts[2] == list.name {
                                found = true;
                                let result = list.delete_task(parts[3].to_string());
                                match result {
                                    Ok(ok) => println!("{}\n", ok),
                                    Err(err) => println!("{}\n", err),
                                }
                                continue;
                            }
                        }
                        if !found {
                            println!("List not found.\n");
                            continue;
                        }
                    }
                    
                    _ => {
                        println!("Usage: delete list <list_name> OR delete task <list_name> <task_name>\n");
                        continue;
                    }
                }
            }
            
            "add" => {
                if parts.len() != 4 || parts[1] != "task" {
                    println!("Usage: add task <list_name> <task_name>\n");
                    continue;
                }
                
                let mut found = false;
                for list in &mut lists {
                    if parts[2] == list.name {
                        found = true;
                        list.add_task(parts[3].to_string());
                        println!("Task added successfully\n");
                        continue;
                    }
                }
                
                if !found {
                    println!("List not found.\n");
                    continue;
                }
                
            }
            
            "complete" => {
                if parts.len() != 4 || parts[1] != "task" {
                    println!("Usage: complete task <list_name> <task_name>\n");
                    continue;
                }
                
                let mut found = false;
                for list in &mut lists {
                    if parts[2] == list.name {
                        found = true;
                        let result = list.complete_task(parts[3].to_string());
                        match result {
                            Ok(ok) => println!("{}\n", ok),
                            Err(err) => println!("{}\n", err),
                        }
                        continue;
                    }
                }
                if !found {
                    println!("List not found.\n");
                    continue;
                }
            }
            
            "show" => {
                if parts.len() != 3 {
                    println!("Usage: show all lists OR show list <list_name>\n");
                    continue;
                }
                
                match parts[1] {
                    "list" => {
                        let mut found = false;
                        for list in &mut lists {
                            if list.name == parts[2] {
                                found = true;
                                list.display();
                                continue;
                            }
                        }
                        if !found {
                            println!("List not found.\n");
                            continue;
                        }
                    }
                    
                    "all" => {
                        if lists.is_empty(){
                            println!("There are no lists\n");
                            continue;
                        }
                        
                        for list in &lists {
                            list.display();
                            println!();
                        }
                        continue;
                    }
                    
                    _ => {
                        println!("Usage: show all lists OR show list <list_name>\n");
                        continue;
                    }
                }
            }
            
            "help" => {
                println!("create list <list_name>\ndelete list <list_name>\nadd task <list_name> <task_name>\ncomplete task <list_name> <task_name>\ndelete task <list_name> <task_name>\nshow list <list_name>\nshow all lists\nexit\n");
            }
            
            "exit" => break,
            
            _ => println!("Unknown command\n"),
        }
    }
    
}


#[allow(dead_code)]
struct Task {
    name: String,
    completed: bool,
}

#[allow(dead_code)]
impl Task {
    fn new(name: String) -> Task {
        Self { name, completed: false }
    }

    fn complete(&mut self) {
        self.completed = true;
    }
}

#[allow(dead_code)]
struct List {
    name: String,
    tasks: Vec<Task>,
}

#[allow(dead_code)]
impl List {
    fn new(name: String,) -> List {
        Self { name, tasks: vec![] }
    }
    
    fn add_task(&mut self, name: String) {
        self.tasks.push(Task::new(name));
    }
    
    fn complete_task(&mut self, name: String) -> Result<String, String> {
        for task in &mut self.tasks {
            if task.name == name {
                task.completed = true;
                return Ok("Task completed successfully".to_string());
            }
        }
        Err("Task does not exist".to_string())
    }
    
    fn delete_task(&mut self, name: String) -> Result<String, String> {
        match self.tasks.iter().position(|t| t.name == name) {
            Some(task) => {
                self.tasks.remove(task);
                Ok("Task deleted successfully".to_string())
            }
            None => {
                Err("Task does not exist".to_string())
            }
        }
    }
    
    fn display(&self){
        println!("List: {}", self.name);
        for task in &self.tasks{
            println!("- {} [{}]", task.name, if task.completed{"x"} else {" "});
        }
        println!();
    }
}