use serde::{Serialize,Deserialize};
use std::env;
use std::fs;


const FILE_NAME: &str = "todos.json";

#[derive(Serialize, Deserialize, Debug)]
struct Task{
   id:u32,
   title:String,
   description:String,
   completed:bool
}

fn main()->Result<(),Box<dyn std::error::Error>>{
   let args: Vec<String>= env::args().collect();
   let mut todos=load_todos()?;
   // todos.push(Task { id: 3, title: String::from("complete chapter3"), completed: true });
   if args.len()==1{
      help_msg();
      return Ok(());
   }
   match args[1].as_str() {
      "add" =>{
         if args.len()<5{
            println!("invalid number of fields for 'add'");
            help_msg();
            return Ok(());
         }
         let title =args[3].to_string();
         let desc=args[4].to_string();
         println!("added new task:\ntitle: {title}\ndescription: {desc}",);
         todos.push(
            Task { 
               id: args[2].parse().unwrap(), 
               title:title, 
               description:desc,
               completed: false 
            }
         );
         save_todos(&todos)?;
      },
      // todos.push(Task { id: 5, title: String::from("chapter 4"), completed: false });
      "list"=> {
         for i in todos.iter(){
            if i.completed!=true{
               println!("[ ] {} {}",i.id,i.title);
            }else {
               println!("[x] {} {}",i.id,i.title);
            }
            println!()
         }
      },
      "delete"=>{
         if args.len()<3{
            println!("enter id duh");
            return Ok(());
         }
         let id : u32=args[2].parse().unwrap();
         todos.retain(|todos|todos.id!=id);
         save_todos(&todos)?;
         println!("{id} has been deleted.")
      },
      "done"=>{
         if args.len()<3{
            println!("enter id duh");
            return Ok(());
         }
         let id : u32=args[2].parse().unwrap();
         for i in &mut todos{
            if i.id==id{
               i.completed=true;
            }
         }
         save_todos(&todos)?;
         println!("{id} has been marked as done.")
      }
      _=>{println!("get help lol")}
   }
   
   Ok(())
}

fn load_todos()->Result<Vec<Task>,Box<dyn std::error::Error>>{

   match fs::read_to_string(FILE_NAME){
      Ok(json)=>{
         let todos= serde_json::from_str(&json)?;
         Ok(todos)
      }
      Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
         Ok(Vec::new())
      }
      Err(e)=>Err(e.into()),
   }
}

fn save_todos(todos:&[Task])->Result<(),Box<dyn std::error::Error>>{
   let json= serde_json::to_string_pretty(todos)?;
   fs::write(FILE_NAME, json)?;
   Ok(())
}

fn help_msg(){
   println!(
r#"commands:
list     :to list the tasks u have.
add      :to add a new task.
          add [id] [task] [description]
delete   :to delete a task.
          delete [id]
done     :to mark as done.
          done [id]"#
   );
}