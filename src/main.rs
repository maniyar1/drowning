#[macro_use] extern crate rocket;
use std::collections::BTreeMap;
use handlebars::Handlebars;
use rocket::http::{Status, ContentType};

#[derive(Responder)]
#[response(status = 200, content_type = "html")]
struct Html(String);

#[get("/")]
fn index() -> Html {
    // create the handlebars registry
    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("t1", "./template.hbs");
    handlebars.register_template_file("header", "./header.hbs");
    handlebars.register_template_file("footer", "./footer.hbs");


    // Prepare some data.
    //
    // The data type should implements `serde::Serialize`
    let mut data = BTreeMap::new();
    let header = handlebars.render("header", &data).unwrap();
    let footer = handlebars.render("footer", &data).unwrap();
    data.insert("header".to_string(), header);
    data.insert("footer".to_string(), footer);
    data.insert("world".to_string(), "Mani".to_string());
    return Html(handlebars.render("t1", &data).unwrap());
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
