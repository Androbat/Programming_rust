use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use  serde::Deserialize;

// 
#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64
}


// Function to handle the response for HTTP GET / request
#[get("/")]
fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        // The r# is a "raw string"
        r#"
         <title>GCD calculator</title>
         <form action="/gcd" method="post">
         
         <input type="text" name="n1"/>
         <input type="text" name="n2"/>
         <button type="submit">Compute GCD</button>
         </form>
    "#,
    )
}

// web::Form<T>, where T is any data structure our form will deserialize from. 
fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
                    .content_type("text/html")
                    .body("Computing the GCD with zero is boring")
    }

    let response = format!("The greatest common divisor of the numbers {} and {} \ is <b>{}</b>\ ", form.n, form.m, gcd(form.n, form.m))
}

#[actix_web::main]
fn main() -> std::io::Result<()> {
    // HttpServer::new() -> creates a server
    // .route("/", web::get().to(get_index))

    // App::new() creates a new App and associates a route with it.
    // web::get().to(get_index) -> handles HTTP GET requests by calling get_index
    HttpServer::new(|| {
        App::new()
            .service(get_index)
            .route("/", web::get().to(get_index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    
}


