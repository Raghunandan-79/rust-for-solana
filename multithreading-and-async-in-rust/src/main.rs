use std::{thread, time::Duration};

use actix_web::{get, App, HttpServer, HttpResponse, Result};
use rouille::{Response, router};
use serde_json::json;
use tokio::sync::mpsc;

fn main() {
    multithreaded_for_loop();
    multithreading_in_rust();
    message_passing();
    thread_http_server_with_cpu_intensive_task();

    // comment above line before running below line
    let _ = first_async_http_server();
}

fn multithreaded_for_loop() {
    const MAX_NUM: u32 = 10000;
    const NUM_OF_THREADS: u32 = 10;
    const PER_ITERATION_COUNT: u32 = MAX_NUM / NUM_OF_THREADS;

    let mut handles: Vec<std::thread::JoinHandle<u32>> = vec![];
    for i in 0..NUM_OF_THREADS {
        let handle = std::thread::spawn(move || {
            let mut sum = 0;
            
            for j in i * PER_ITERATION_COUNT..(i + 1) * PER_ITERATION_COUNT {
                sum += j;
            }

            return sum;
        });

        handles.push(handle);
    }

    let mut sum: u32 = 0;
    for handle in handles {
        let ans: u32 = handle.join().unwrap();
        sum = sum + ans;
    }

    println!("{}", sum);
}

fn multithreading_in_rust() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("hi number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi no {i} from main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

#[tokio::main]
async fn message_passing() {
    let (tx, mut rx) = mpsc::channel(10);

    tokio::spawn(async move {
        tx.send("hi from task").await.unwrap();
    });

    // Doesn't block thread, just awaits asynchronously
    if let Some(msg) = rx.recv().await {
        println!("Got: {}", msg);
    }
}

fn thread_http_server_with_cpu_intensive_task() {
    println!("Server on http://127.0.0.1:8080");

    rouille::start_server("127.0.0.1:8080", move |request| {
        router!(request,
            (GET) (/) => {
                let sum = calculate_sum(1000_000_000);

                Response::json(&serde_json::json!({
                    "sum": sum,
                }))
            },
            _ => Response::empty_404()
        )
    })
}   

fn calculate_sum(n: i64) -> i64 {
    let mut sum: i64 = 0;
    
    for i in 1..=n {
        sum += i;
    }

    return sum;
}

#[actix_web::main]
async fn first_async_http_server() -> std::io::Result<()> {
    println!("Server on http://127.0.0.1:8081");

    HttpServer::new(|| {
        App::new()
            .service(read_file)  
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

#[get("/")]
async fn read_file() -> Result<HttpResponse> {
    let file_contents = std::fs::read_to_string("a.txt")
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "file_contents": file_contents,
    })))
}
