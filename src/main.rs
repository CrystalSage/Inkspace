#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;
use rocket_dyn_templates::context;

use std::env;

fn getfile() -> String{
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    return filename.to_string();
}

#[get("/")]
fn index() -> Template {
    let fname: String = getfile();
    Template::render(fname , context!{
        name: "Bourbon",
    })
}

#[get("/list")]
fn list() -> Template {
    Template::render("list" , context!{ })
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, list])
        .attach(Template::fairing())
}
