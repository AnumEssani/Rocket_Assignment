#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::request::Form;
use rocket::response::NamedFile;


#[derive(Debug, FromForm)]
struct Task {
     num: u32,
}

#[post("/", data = "<task>")]
fn post(task: Form<Task>)->String{
    format!("Number is = {} after add 50 = {}",task.num,task.num + 50)
}

#[get("/")]
fn index()-> Option<NamedFile> {
    NamedFile::open("static/index.html").ok()
}
#[get("/<num>")]
fn get(num:usize) -> String {
    format!("you input {} after add 50 now number is {}",num,num+50)
}

fn main() {
    rocket::ignite().mount("/", routes![index,post,get]).launch();
}