use std::collections::HashMap;
use std::io;
#[derive(Debug, Clone)]
pub struct Student {
    id : String,
    name: String,
    age: u32,
}


pub struct  StudentList {
    class : HashMap<String, Student>,
}

impl StudentList {

    fn new() -> Self {
        Self {
            class : HashMap::new(),
        }
    
    }

    fn add(&mut self, student: Student) {
        if self.class.contains_key(&student.id) {
            println!("Student already exists");
            return;
        }
        self.class.insert(student.id.to_string(), student);
        
    }


    fn remove(&mut self, id: &str) -> bool {
        self.class.remove(id).is_some()
    }

    fn edit(&mut self, id: &str, age: u32, name: String) -> bool {
        match self.class.get_mut(id) {
            Some(student) => {
                student.age = age;
                student.name = name;
                true
            }
            None => false,
        }
    }

    fn isExist(&mut self, id: &str) -> bool {
        if self.class.contains_key(id) {
            true;
        }
        false
    }
}

mod manager {
    use crate::*;

    pub fn add_student(list : &mut StudentList) {
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };

        let id = match get_input() {
            Some(id) => {
                if list.isExist(&id) {
                    return;
                } else {
                    id
                }
            }
            None => return,
        };

        let age = match get_int() {
            Some(age) => {
                if age < 0 && age > 100 {
                    return;
                } else {
                    age as u32
                }
            }
            None => return,
        };

        list.add(Student { id, name, age});
    } 

    pub fn view_student(list : &mut StudentList) {
        for student in list.class.values() {
            print!("{:#?}", student);
        }
    }

    pub fn remove_student(list : &mut StudentList) {
        let id = match get_input() {
            Some(take) => {
                if list.isExist(&take) {
                    take
                } else {
                    print!("Sorry it is not exist");
                    return;
                }
                
            }
            None => return,
        }; 
        list.remove(&id);
    }

    pub fn edit_student(list: &mut StudentList) {
        let id = match get_input() {
            Some(take) => take,
            None => return,
        }; 

        let name = match get_input() {
            Some(name) => name,
            None => return,
        };


        let age = match get_int() {
            Some(age) => {
                if age < 0 && age > 100 {
                    return;
                } else {
                    age as u32
                }
            }
            None => return,
        };

        StudentList::edit(list, &id, age, name);
    }
}

enum Manager {
    AddStudent,
    ViewStudent,
    EditStudent,
    DeleteStudent,
}

impl Manager {
    fn show() {
        println!("");
        println!("== Manager Panel ==");
        println!("");
        println!("1. Add Student");
        println!("2. View Students");
        println!("3. Edit Student");
        println!("4. Delete Student");
        println!("");
        println!("Please Enter Your Choice:");
        println!("");
    }

    fn choice(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::AddStudent),
            "2" => Some(Manager::ViewStudent),
            "3" => Some(Manager::EditStudent),
            "4" => Some(Manager::DeleteStudent),
            _ => None,
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("please try again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

fn get_int() -> Option<i32> {
    println!("Enter Age of Student");
    let input = match get_input() {
        Some(input) => input,
        None => return None,
    };
    let parsed_input: Result<i32, _> = input.parse();
    match parsed_input {
        Ok(input) => Some(input),
        Err(_) => None,
    }
}
    
fn run_program() {
    let mut studentList = StudentList::new();
    loop {
        Manager::show();
        let input = get_input().expect("Please enter your data");
        match Manager::choice(&input) {
            Some(Manager::AddStudent) => manager::add_student(&mut studentList),
            Some(Manager::ViewStudent) => manager::view_student(&mut studentList),
            Some(Manager::DeleteStudent) => manager::remove_student(&mut studentList),
            Some(Manager::EditStudent) => manager::edit_student(&mut studentList),
            _ => return,
        }
    } 
}




fn main() {
    run_program();
    print!("Good bye");
}