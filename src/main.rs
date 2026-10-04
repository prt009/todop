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
   if args.len()==1{
      help_msg();
      return Ok(());
   }
   match args[1].as_str() {
      "add" =>{
         if args.len()<5{
            println!("invalid number of fields for 'add'");
            println!("add [id] [task] [description]");
            return Ok(());
         }
         let title =args[3].to_string();
         let desc=args[4].to_string();
         let idx:u32=args[2].parse().unwrap();
         for i in &todos{
            if i.id==idx{
               println!("id already exists. to edit run edit command.");
               return Ok(());
            }
         }
         println!("added new task:\ntitle: {title}\ndescription: {desc}",);
         todos.push(
            Task { 
               id: idx, 
               title:title, 
               description:desc,
               completed: false 
            }
         );
         save_todos(&todos)?;
      },
      "edit" =>{
         if args.len()<4{
            println!("invalid number of fields for 'edit'");
            return Ok(());
         }
         let id:u32=args[2].parse().unwrap();
         match args[3].as_str(){
            "t"=>{
               if args.len()>5{
                  println!("invalid number of fields for a title edit");
                  return Ok(());
               }
               let title =args[4].to_string();
               edit_title(&mut todos, id, &title);
            }
            "d"=>{
               if args.len()<5{
                  println!("invalid number of fields for desc edit");
                  return Ok(());
               }
               let desc=args[4].to_string();
               edit_desc(&mut todos, id, &desc);
            }
            "td"=>{
               if args.len()<6{
                  println!("invalid number of fields for td edit ");
                  return Ok(());
               }
               let title =args[4].to_string();
               let desc=args[5].to_string();
               edit_title(&mut todos, id, &title);
               edit_desc(&mut todos, id, &desc);
            },
            _=>{println!("invalid command.. \nedit [id] [td|t|d] [args..]");return Ok(())}
         }
         save_todos(&todos)?;
         println!("edited.")
       
      },
      "list"=> {
         for i in todos.iter(){
            if i.completed!=true{
               println!("[ ] {} {}",i.id,i.title);
            }else {
               println!("[x] {} {}",i.id,i.title);
            }
            println!("        :{}",i.description);
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

fn edit_title(todos:&mut [Task],id:u32,new_t: &str){
   for i in todos{
      if id ==i.id{
         i.title= String::from(new_t);
      }
   }
}
fn edit_desc(todos:&mut [Task],id:u32,new_d: &str){
   for i in todos{
      if id ==i.id{
         i.description= String::from(new_d);
      }
   }
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
edit     :to edit a task with id.
          edit [id] [td|t|d] [args..]
            td: [td] [task] [desc]  edit both title and description.
            t : [t] [task]          edit only title.
            d : [d] [desc]          edit only description.
delete   :to delete a task.
          delete [id]
done     :to mark as done.
          done [id]"#
   );
}