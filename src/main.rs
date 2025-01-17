use std;
use std::collections::HashMap;

// use webserver::thread_pool::ThreadPool;
use webserver::response::{
    make_response,
    Template,
};


const TEMPLATE: Template<'_> = Template { root: "templates" };


use webserver::app::App;
use webserver::router::Router;
use webserver::run::run_multithread;
use webserver::request::content_type::ContentType;


fn main() {
    let mut app: App = App::new();

    let mut router = Router::new();

    router.get("/favicon.ico", |(request, path_args)| {
        make_response(404, String::from("NOT FOUND"))
    });

    router.post("/api/user", |(request, path_args)| {
        // read json
        println!("call user");
        println!("{request}");
        use std::{thread, time};
        thread::sleep(time::Duration::from_secs(5));
        match &request.body {
            ContentType::Json(v) => {
                make_response(200, v.dump())
            },
            _ => {
                make_response(404, String::from("NOT FOUND"))
            },
        }
    });

    router.get("/{file_name}", |(request, path_args)| {
        let args = HashMap::<String, String>::new();
        println!("{request}");
        println!("{path_args:?}");
        println!("{:?}", request.query);
        
        let fname = &path_args["file_name"];
        if let Ok(resp) = TEMPLATE.make_response(200, &fname, &args) {
            resp
        } else if let Ok(resp) = TEMPLATE.make_response(400, "404.html", &args) {
            resp
        } else {
            make_response(404, String::from("NOT FOUND"))
        }
    });

    app.include_router("", Box::new(router));
    match run_multithread(app, "127.0.0.1", 7878, 4) {
        Ok(_) => {},
        Err(e) => println!("{e}"),
    }
}
