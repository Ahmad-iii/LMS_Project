use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use serde::Serialize;
struct AppState {
    counter: Mutex<i32>, // Shared counter state
}

// Handler for the counter page
async fn counter_page(data: web::Data<AppState>) -> impl Responder {
    let count = *data.counter.lock().unwrap();
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Counter</title>
            <script src="https://cdn.tailwindcss.com"></script>
            <script>
                async function updateCounter(action) {{
                    let response = await fetch('/' + action, {{ method: 'POST' }});
                    let data = await response.json();
                    document.getElementById("counter").innerText = data.count;
                }}
            </script>
        </head>
        <body class="flex flex-col items-center justify-center h-screen bg-gray-100">
            <h1 class="text-4xl font-bold mb-4">Counter: <span id="counter">{}</span></h1>
            <div class="space-x-4">
                <button onclick="updateCounter('increment')" class="px-4 py-2 bg-blue-500 text-white rounded">Increment</button>
                <button onclick="updateCounter('decrement')" class="px-4 py-2 bg-red-500 text-white rounded">Decrement</button>
                <button onclick="updateCounter('reset')" class="px-4 py-2 bg-gray-500 text-white rounded">Reset</button>
            </div>
        </body>
        </html>
        "#,
        count
    );
    HttpResponse::Ok().content_type("text/html").body(html)
}



#[derive(Serialize)]
struct CounterResponse {
    count: i32,
}

async fn increment(data: web::Data<AppState>) -> impl Responder {
    let mut count = data.counter.lock().unwrap();
    *count += 1;
    HttpResponse::Ok().json(CounterResponse { count: *count })
}

async fn decrement(data: web::Data<AppState>) -> impl Responder {
    let mut count = data.counter.lock().unwrap();
    *count -= 1;
    HttpResponse::Ok().json(CounterResponse { count: *count })
}

async fn reset(data: web::Data<AppState>) -> impl Responder {
    let mut count = data.counter.lock().unwrap();
    *count = 0;
    HttpResponse::Ok().json(CounterResponse { count: *count })
}


// Handler for the "Hello World" page
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body("<h1>Hello World</h1>")
}

// Main function to configure and start the server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/", web::get().to(counter_page))
            .route("/increment", web::post().to(increment))
            .route("/decrement", web::post().to(decrement))
            .route("/reset", web::post().to(reset))
            .route("/hello", web::get().to(hello_world))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
