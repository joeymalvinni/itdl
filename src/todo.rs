use std::fs;

pub struct Todo {
    todo: Vec<String>,
    done: Vec<String>,
}

impl Todo {
    pub fn new() -> Todo {
        Todo {
            todo: vec![],
            done: vec![],
        }
    }

    // creates todo struct from file
    pub fn from(file_name: &str) -> Todo {
        let content = fs::read_to_string(file_name).expect("Error finding file.");
        let lines = content.lines();

        let mut todo_vec: Vec<String> = vec![];
        let mut done_vec: Vec<String> = vec![];
        
        
        for line in lines.into_iter() {
            if line.to_lowercase().starts_with("done: ") {
                todo_vec.push(line[6..].trim().to_string());
            } else if line.to_lowercase().starts_with("todo: ") {
                done_vec.push(line[6..].trim().to_string());
            }
        };

        Todo {
            todo: todo_vec,
            done: done_vec,
        }
    }

    pub fn remove_from_todo(&mut self, index: usize) -> String {
        self.todo.swap_remove(index)
    }

    pub fn remove_from_done(&mut self, index: usize) -> String {
        self.done.swap_remove(index)
    }

    pub fn add_to_todo(&mut self, message: String) {
        self.todo.push(message);
    }

    pub fn add_to_done(&mut self, message: String) {
        self.done.push(message);
    }

    // removes a TODO item from the todo vec and pushes it to the done vec
    pub fn mark_as_done(&mut self, index: usize) {
        let element = self.remove_from_todo(index);

        self.add_to_done(element);
    }

    pub fn mark_as_todo(&mut self, index: usize) {
        let element = self.remove_from_done(index);

        self.add_to_todo(element);
    }

    // returns a String of the done elements
    pub fn collect_done_md(&mut self) -> Vec<String> {
        self.done
            .as_mut_slice()
            .into_iter()
            .map(|item| format!("- [x] {}", item.to_string()))
            .collect::<Vec<_>>()
    }

    pub fn collect_todo_md(&mut self) -> Vec<String> {
        self.todo
            .as_mut_slice()
            .into_iter()
            .map(|item| format!("- [ ] {}", item.to_string()))
            .collect::<Vec<_>>()
    }

    pub fn collect_all_md(&mut self) -> Vec<String> {
        let mut list = self.collect_todo_md();
        list.append(&mut self.collect_done_md());
        list
    }
}
