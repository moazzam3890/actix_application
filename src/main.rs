use actix_web::{web, App, Responder, HttpServer};
use std::sync::Mutex;

async fn index_0() -> impl Responder{ //Request handler that return a response
    "Hello through .route!"
}

struct appstate {
    app_name: String,
}
//Data has been accessed by using Data<T> (T is a data type: here struct) 
async fn index_1(data: web::Data<appstate>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}", app_name)
}

#[actix_rt::main]
async fn main () -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
            .service(web::scope("/app")//Configure scope for common root path
        .route("/index_0", web::get() //Create route with get http method gaurd
        .to(index_0)))// create a new route and add handler
        .data(appstate{ // Application data is set and sent to request handler though struct
            //at line 8
            app_name: String::from("Actix-Web"),
        })
        //METHOD | PATH  |GET METHOD | FROM REQUEST (request handler index_1)
        .route("/index_1", web::get().to(index_1))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}