use std::env;
use std::fmt;
use serde_json::Result;
use std::fs;
use std::any::type_name;
use std::collections::HashMap;
use std::io::Write;

fn print_type_of<T> (_:&T){
    println!("{}", std::any::type_name::<T>());
}

fn done(id: usize) -> bool {
    let mut maped: HashMap<String, Vec<String>> = get_tasks();

    if let Some(task) = maped.get_mut("task") {
        if task.len() >= id {
            let removed : String = task.remove(id-1);
            let new_list = maped.get_mut("completed").unwrap();
            new_list.push(removed);
        }else{
            return false;
        };
    };
  
    // write new updates back to jason file

    let mut file = fs::File::create("src/task.json")
        .expect("Could not create file");
    let json_data = serde_json::to_string_pretty(&maped).unwrap();

    file.write_all(json_data.as_bytes())
        .expect("Could not write to file");
    
    true

}

fn get_tasks() -> HashMap<String, Vec<String>>{
    // create hashmap to store list
    


    // get list and 
    let json_data = match fs::read_to_string("src/task.json") {
        Ok(data) => data,
        Err(_) => panic!("Something went wrong reading src/task.json")
    };

    let maped: HashMap<String, Vec<String>> = serde_json::from_str(&json_data)
        .expect("something went wrong with parsing the json object");

    maped
}

fn list() {

    let maped: HashMap<String, Vec<String>> = get_tasks();


    for (key,value) in &maped {
        println!("========{key}=======");        

        for (i,item) in value.iter().enumerate(){
            println!("{}. {}",i + 1, item)
        }
    };

}
// fn add_task() -> String{

// }
fn main() {

    let args: Vec<String> = env::args().collect();
    let app = &args[1];
    let command = &args[2];

    if app != "todo" {
        println!("Incorrect program name: {app}");
    }else{
       
        // program name is correct
        if args.len() == 4 {
            
            if command == "done" {
                let argument: usize = match args[3].trim().parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => panic!("Please enter a number here"),
                };
                done(argument);
            };

        }else if args.len() == 3{
            
            //  this includes things like view list
            list();
        };
        
    };
}
