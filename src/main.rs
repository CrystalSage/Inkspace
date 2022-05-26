#[macro_use] extern crate rocket;
use rocket_dyn_templates::Template;
use rocket_dyn_templates::context;

#[get("/")]
fn index() -> Template {
    Template::render("index", context!{
        name: "Bourbon",
        quote: "The weak links of strong chain"
    })
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
