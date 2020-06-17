use actix_web::{web, App, Responder, HttpServer};

async fn index_0() -> impl Responder{ //Request handler that return a response
    "Hello World!"
}

#[actix_rt::main]
async fn main () -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
            .service(web::scope("/app")//Configure scope for common root path
        .route("/index", web::get() //Create route with get http method gaurd
        .to(index_0)))// create a new route and add handler
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}