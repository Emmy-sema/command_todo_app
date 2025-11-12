use std::env;
use std::io::Write;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::Path;


#[derive(Serialize, Deserialize, Debug)]
struct Task{
    name: String,
    completed: bool,
    // #[serde(with = "rc_refcell_serde")]
    // children: Vec<Rc<RefCell<Task>>>,
    children:Vec<Task>,
    number_of_tasks_completed: i8,
    number_of_tasks: i8
}


#[derive(Serialize, Deserialize, Debug)]
struct User{
    // #[serde(with = "rc_refcell_serde")]
    // children: Vec<Rc<RefCell<Task>>>,
    children:Vec<Task>,
    number_of_tasks: i8,
    number_of_tasks_completed: i8    
}


impl Task{
    
    fn display(&self,level:i8,idx:usize){

        // let mut ouput: String = String::new();
        let space = " ".repeat(level as usize);
        println!("{}[{}] {}", space, idx, self.name);

        for i in 0..self.children.len(){
            let child = &self.children[i];
            child.display(level + 1,i as usize);
        }


    }
}
impl User{
    fn new() -> Self{
       User{
        children:vec![],
        number_of_tasks_completed: 0,
        number_of_tasks: 1
        // todo: Task{
        //         name: name,
        //         completed: completed,
        //         parent: parent,
        //         children: vec![],
        //         number_of_tasks_completed: number_of_tasks_completed,
        //         number_of_tasks: number_of_tasks
        //     }
        }
    }

    fn add_task(&mut self, name: String, id:Option<String>) -> Result<String,String> {

        // let new_task = Rc::new(RefCell::new(Task {
        //     name: name,
        //     completed: false,
        //     // parent: self,
        //     children: vec![],
        //     number_of_tasks_completed: 0,
        //     number_of_tasks: 1       
        // }));

        let new_task = Task {
            name: name,
            completed: false,
            // parent: self,
            children: vec![],
            number_of_tasks_completed: 0,
            number_of_tasks: 1 
        };

        if self.children.len() == 0{
            self.children.push(new_task);
            self.number_of_tasks += 1;
            return Ok("Task added Successfully".to_string())
        }
        match id{
            Some(id_str) => {
                let parts: Vec<&str> = id_str.split(".").collect();
            
                if parts.is_empty(){
                    return Err(format!("No task selected"));
                }                

                let mut indices: Vec<usize> = vec![];
                for part in parts{
                    match part.trim().parse::<usize>(){
                        Ok(id) => indices.push(id),
                        Err(_) => return Err(format!("Make sure your id pointer are all i8"))
                    }
                    
                };

                let mut current = &mut self.children;
                for &i in &indices[..indices.len().saturating_sub(1)]{
                    
                    if i >= current.len(){
                        return Err(format!("Make sure your id pointer are all i8"));
                    }

                    current = &mut current[i].children;
                    
                }

                let last_idx = *indices.last().unwrap_or(&0);
                if last_idx > current.len() {
                    return Err("Invalid task position.".to_string());
                }

                current.insert(last_idx, new_task);
                self.number_of_tasks += 1;
            },

            None =>{
                self.children.push(new_task);
                self.number_of_tasks += 1
            }
        }
      
        Ok("Task added successfully.".to_string())

        
    }

    fn display(&self){

        for i in 0..self.children.len(){
            let task = &self.children[i];
            task.display(0,i);
        }
    }

    

    
}


fn get_data() -> Result<User,String>{
    let path = Path::new("src/task.json");

    if path.exists() && path.is_file(){
        let data = match fs::read_to_string("src/task.json"){
            Ok(f) => {
                f
            },
            Err(err) => {
                
                return Err(format!("{:?}",err));
            }
        };

        let user: User = match serde_json::from_str(&data){
            Ok(res) => {
                res
            },
            Err(_)=>{
                return Ok(User::new());
                
            }
        };

        Ok(user)
    }else {
        let _ = fs::File::create("src/task.json").expect("Was not able to create file");
        return Ok(User::new());
    }

   
}

fn write_to_storage(user:User) -> Result<String,String>{
    let mut file = match fs::File::create("src/task.json"){
        Ok(f) =>{
            f
        },
        Err(err) => {
            return Err(format!("{:?}",err));
        }
    };
    let json_data: String = match serde_json::to_string_pretty(&user){
        Ok (data)=> {
            data
        },
        Err(err) =>{
            return Err(format!("{:?}",err));
        }
    };

 
    match file.write(json_data.as_bytes()){
        Ok(_) => {
            return Ok("Successfully wrote to storage".to_string())
        },
        Err(err)=> {
            return Err(format!("{:?}",err));
        }
    };
    
}
fn main(){ 

    let args: Vec<String> = env::args().collect();
    if args.len() == 1{
        println!("No arguemnts passed");
        println!("Type: 'cargo run -- /help', for help");
        return
    };
    let query = &args[1];
    match query.as_str() {
        "add" => {
            if args.len() < 4 {
                println!("Usage: cargo run -- add <position> '<task name>'");
                return;
            };
            let path: String = args[2].clone();
            let name: String = args[3].clone();

            let mut user:User = match get_data(){
                Ok(user) => {
                    user
                },
                Err(err) => {
                    return println!("{:?}",err);
                }
            };
            match user.add_task(name, Some(path)) {
                Ok(res) => {
                    println!("{:?}",res);
                },
                Err(err) => {
                    println!("{:?}",err);
                }
            }
            let _ = write_to_storage(user);
        },
        "display" =>{
            match get_data(){
                Ok(user) => {
                    user.display();
                },
                Err(err) => {
                    return println!("{:?}",err);
                }
            };            
        },
        "/help" =>{
           println!(
                "
                Usage Examples:

                Add to list:
                cargo run -- add 0 'Breath'

                    Notes:
                    - 'add' is the command.
                    - '0' specifies where to add the task.
                    - 'Breath' is the task itself.

                    Adding subtasks:
                    - '0.0' will add a subtask under task 0.
                    - Using '0.1' when there is no task 0 or subtask 1 will fail.

                Display list:
                cargo run -- display
                "
            );
        },
        _ => println!("Not a valid command. Type 'cargo run -- /help' for help."),
    }

    
}