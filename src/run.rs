use std::net::TcpListener;
use std::sync::{Arc, Mutex};

use crate::app::App;
use crate::thread_pool::ThreadPool;


fn get_listener(host: &str, port: usize) -> Result<TcpListener,String> {
    let ip_port = format!("{host}:{port}");
    match TcpListener::bind(&ip_port) {
        Ok(v) => {
            println!("Listening to {} ...", &ip_port);
            Ok(v)
        },
        Err(_) => {
            Err(String::from("Cannot bind {ip_port}, it is already used by other process."))
        },
    }
}


pub fn run(app: App, host: &str, port: usize) -> Result<(),String> {
    let listener = get_listener(host, port)?;

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(v) => v,
            Err(_) => continue,
        };
        app.handle_connection(stream);
    }
    Ok(())
}


pub fn run_multithread(app: App<'static>, host: &str, port: usize, threads: usize) -> Result<(),String> {
    let listener = get_listener(host, port)?;
    let pool = ThreadPool::new(threads);
    let wrapped_app = Arc::new(Mutex::new(app));

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(v) => v,
            Err(_) => continue,
        };
        
        let app_cloned = Arc::clone(&wrapped_app);
        pool.execute(move || {
            let _app = app_cloned.lock().unwrap();
            _app.handle_connection(stream);
        });
    }
    Ok(())
}
