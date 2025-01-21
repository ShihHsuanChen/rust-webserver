use std;
use std::collections::HashMap;
use num_cpus;

use clap::Parser;

use webserver::response::{
    make_text_response,
    Template,
};
use webserver::app::App;
use webserver::router::Router;
use webserver::run::run_multithread;
use webserver::request::content_type::ContentType;


const TEMPLATE: Template<'_> = Template { root: "templates" };


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ArgumentParser {
    /// IP address. For example: 127.0.0.1, 0.0.0.0, localhost
    #[arg(short, long, default_value_t = String::from("localhost"))]
    ip: String,

    /// port number
    #[arg(short, long, default_value_t = 5000)]
    port: usize,

    /// number of threads
    #[arg(short, long, default_value_t = num_cpus::get())]
    nthreads: usize,
}


fn get_ui_router() -> Router<'static> {
    let mut router = Router::new();

    router.get("/favicon.ico", |(request, path_args)| {
        Ok(Box::new(make_text_response(404, String::from("NOT FOUND"))?))
    });

    router.get("/{file_name}", |(request, path_args)| {
        let args = HashMap::<String, String>::new();
        println!("{request}");
        println!("{path_args:?}");
        println!("{:?}", request.query);
        
        let fname = &path_args["file_name"];
        if let Ok(resp) = TEMPLATE.make_response(200, &fname, &args) {
            Ok(Box::new(resp))
        } else if let Ok(resp) = TEMPLATE.make_response(400, "404.html", &args) {
            Ok(Box::new(resp))
        } else {
            Ok(Box::new(make_text_response(404, String::from("NOT FOUND"))?))
        }
    });
    router
}

fn get_api_router() -> Router<'static> {
    let mut router = Router::new();

    router.post("/user", |(request, path_args)| {
        // read json
        println!("call user");
        println!("{request}");
        use std::{thread, time};
        thread::sleep(time::Duration::from_secs(5));
        match &request.body {
            ContentType::Json(v) => {
                Ok(Box::new(make_text_response(200, v.dump())?))
            },
            _ => {
                Ok(Box::new(make_text_response(404, String::from("NOT FOUND"))?))
            },
        }
    });
    router
}

fn main() {
    let args = ArgumentParser::parse();
    let mut app: App = App::new();

    app.include_router("", Box::new(get_ui_router()));
    app.include_router("/api", Box::new(get_api_router()));

    match run_multithread(app, &args.ip, args.port, args.nthreads) {
        Ok(_) => {},
        Err(e) => println!("{e}"),
    }
}
