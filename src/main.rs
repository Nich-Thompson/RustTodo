use rocket::response::content::Html;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
fn index2() -> &'static str {
    "Hello, world! 2"
}

#[get("/")]
fn index3() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8">
                <title>My App</title>
                <link data-trunk rel="css" href="style.css">
            </head>
            <body>
                <div>
                    <button>Click me!</button>
                </div>
            </body>
        </html>
    "#)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .mount("/2", routes![index2])
    .mount("/3", routes![index3])
}