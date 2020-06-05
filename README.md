# IOT Class TASK 
Create a server which get a number in parameter and return the number + 50

first add rocket in toml file
```
[dependencies]
rocket = "0.4.4"
```
then import in main.rs file
```
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::request::Form;
use rocket::response::NamedFile;
```

Create Struct to get data from html form
```
#[derive(Debug, FromForm)]
struct Task {
     num: u32,
}
```
creat testPost function for post request
where Path is Index or "/" and data = prametor which we pass in function
parameter task data type is Task Struct and pass it through Rocket From
this function is returning a string from function
```
#[post("/", data = "<task>")]
fn testPost(task: Form<Task>)->String{
    format!("Number is = {} after add 50 = {}",task.num,task.num + 50)
}
```
creat index function for get request
where Path is Index or "/" 
this fuction return a option enum with data type of NamedFile Struct which return result
```
#[get("/")]
fn index()-> Option<NamedFile> {
    NamedFile::open("static/index.html").ok()
}
```
creat num function for get request
where Path is Index or "/"  and <num> is expeting a parametor in url 
this fuction return a string
```
#[get("/<num>")]
fn num(num:u32)-> String {
    format!("Number is = {} after add 50 = {}",num ,num + 50)
}
```
creat main function for lucnhing rocket
```
fn main() {
    rocket::ignite().mount("/", routes![index,testPost,num]).launch();
}
```

Now creat a folder in main Name Static 

now create html file name index.html
and craet a form in html with one input feild and submit button and form action = "/" and method ="post"

```

<form action="/" method="post" accept-charset="utf-8">
    <label>Number:
        <input type="number" name="num" id="num" placeholder="Enter a Number">
    </label>
    <br /><br />
    <label>Submit:
        <input type="submit" value="submit">
    </label>
    <br /><br />
</form>
```

run the code and rocket launch to moon!!! ....
