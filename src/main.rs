use actix_web::{web, App, Responder, HttpServer};
use std::sync::Mutex;

async fn index_0() -> impl Responder{ //Request handler that return a response
    "Hello through .route!"
}
//App State
struct appstate {
    app_name: String,
}
//Data has been accessed by using Data<T> (T is a data type: here struct) 
async fn index_1(data: web::Data<appstate>) -> String {
    let app_name = &data.app_name;
    format!("Hello {}", app_name)
}
//ID-Card State
struct id_card {
    id_no: String,
    name: String,
    batch : String,
    quarter : String,
}

async fn index_2(id: web::Data<id_card>) -> String {
    let id_no = &id.id_no;
    let name = &id.name;
    let batch = &id.batch;
    let quarter = &id.quarter;
    format!("ID Number : {}, Name : {}, Batch : {}, Quarter : {}", id_no, name, 
    batch, quarter)
}

//Shared Mutable State:
struct AppStateMutable {
    //Mutex is neccessary to Mutate safely across threads
    counter : Mutex<i32>,
}

async fn index_3(data: web::Data<AppStateMutable>) -> String {
    //get the counters MutexGaurd
    let mut counter = data.counter.lock().unwrap();
    //access counter inside Mutex Gaurd
    *counter += 1;
    format!("Request Number : {}", counter)
}

#[actix_rt::main]
async fn main () -> std::io::Result<()>{
    let counter = web::Data::new(AppStateMutable{
        counter: Mutex::new(0),
    });         //  move counter into the closure {}
    HttpServer::new(move||{
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
        .data(id_card{ //Data is set and sent to request handler through struct at line 16 
            id_no: String::from("123"),
            name: String::from("Moazzam Adil Khan"),
            batch: String::from("2"),
            quarter: String::from("3"),
        })
        //METHOD | PATH  |GET METHOD | FROM REQUEST (request handler index_2)
        .route("/index_2", web::get().to(index_2))
        // register the created data
        .app_data(counter.clone())

        .route("/index_3", web::get().to(index_3))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}