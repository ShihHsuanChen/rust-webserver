use std::collections::HashMap;

use regex::Regex;


use crate::http::{
    Method,
    get_method_from_str,
};
use crate::response::{MakeResponse, Response};
use crate::request::Request;


pub type ResponseResult = Result<Box<dyn MakeResponse>,String>;
type Callback = Box<dyn Fn((&Request, HashMap<String,String>)) -> ResponseResult + Send + 'static>;

struct Route<'a> {
    method: Method<'static>,
    path: &'a str,
    re: Regex,
    f: Callback,
}

impl<'a> Route<'a> {
    fn new<F>(path: &'a str, method: &str, f: F) -> Self
    where
        F: Fn((&Request, HashMap<String,String>)) -> ResponseResult + Send + 'static 
    {
        // validate path 
        if !path.starts_with("/") {
            panic!(
                "path must be empty or starting with '/', got '{path}'"
            );
        }
        // extract path arguments:
        //   path format: /aaa/bbb/{id}...
        //   regex: /aaa/bbb/(?<id>[^\/\#\?]+)/...$
        //   replace { to (?<
        //   replace } to >[^\/\#\?]+)
        let pattern = String::from(path)
            .replace(".", "\\.")
            .replace("{", "(?<")
            .replace("}", ">[^/#?]+)");
        let pattern = format!("^{pattern}$");
        let re = Regex::new(&pattern).unwrap();
        Route {
            method: get_method_from_str(method).unwrap(),
            path,
            re,
            f: Box::new(f),
        }
    }

    fn execute(&self, path: &str, request: &Request) -> Option<ResponseResult> {
        println!("54: matching {} with {}:", path, self.re.as_str());
        let path_args: HashMap<String, String> = match self.re.captures(path) {
            Some(caps) => {
                self.re.capture_names()
                    .flatten()
                    .filter_map(|k| Some(
                        (k.to_string(), caps.name(k)?.as_str().to_string())
                    ))
                    .collect()
            },
            None => return None,
        };
        println!("{path_args:?}");
        Some(self.f.as_ref()((request, path_args)))
        // self.f.as_ref()()
    }
}


// type EndPointsMap = HashMap<String, HashMap<Method<'static>, Route<'static>>>;
// type EndPointsMap = HashMap<String, Vec<Route<'static>>>;
/// {prefix -> routes}
type EndPointsMap<'a> = HashMap<String, Vec<Box<Route<'a>>>>;
/// {prefix -> routers}
type RoutersMap<'a> = HashMap<String, Vec<Box<Router<'a>>>>;


pub struct Router<'a> {
    routers: RoutersMap<'a>,
    endpoints: EndPointsMap<'a>,
}

impl<'a> Router<'a> {
    pub fn new() -> Self {
        let routers = HashMap::<String, Vec<Box<Router>>>::new();
        let endpoints = EndPointsMap::new();
        Self {
            routers,
            endpoints,
        }
    }

    pub fn get<F>(&mut self, path: &'a str, f: F)
    where
        F: Fn((&Request, HashMap<String,String>)) -> ResponseResult + Send + 'static 
    {
        self.add_route(Box::new(Route::new(path, "GET", f)));
    }

    pub fn post<F>(&mut self, path: &'a str, f: F)
    where
        F: Fn((&Request, HashMap<String,String>)) -> ResponseResult + Send + 'static 
    {
        self.add_route(Box::new(Route::new(path, "POST", f)));
    }

    pub fn put<F>(&mut self, path: &'a str, f: F)
    where
        F: Fn((&Request, HashMap<String,String>)) -> ResponseResult + Send + 'static 
    {
        self.add_route(Box::new(Route::new(path, "PUT", f)));
    }

    pub fn patch<F>(&mut self, path: &'a str, f: F)
    where
        F: Fn((&Request, HashMap<String,String>)) -> ResponseResult + Send + 'static 
    {
        self.add_route(Box::new(Route::new(path, "PATCH", f)));
    }

    pub fn delete<F>(&mut self, path: &'a str, f: F)
    where
        F: Fn((&Request, HashMap<String,String>)) -> ResponseResult + Send + 'static 
    {
        self.add_route(Box::new(Route::new(path, "DELETE", f)));
    }

    pub fn option<F>(&mut self, path: &'a str, f: F)
    where
        F: Fn((&Request, HashMap<String,String>)) -> ResponseResult + Send + 'static 
    {
        self.add_route(Box::new(Route::new(path, "OPTION", f)));
    }

    pub fn head<F>(&mut self, path: &'a str, f: F)
    where
        F: Fn((&Request, HashMap<String,String>)) -> ResponseResult + Send + 'static 
    {
        self.add_route(Box::new(Route::new(path, "HEAD", f)));
    }

    fn add_route(&mut self, route: Box<Route<'a>>) {
        // TODO: FAIL: error[E0599]: the method `insert` exists for mutable reference `&mut HashMap<Method<'static>, Route<'static>>`, but its trait bounds were not satisfie
        // let method = get_method_from_str(method).unwrap();
        // let map = self.endpoints
        //     .entry(String::from(route.path))
        //     .or_insert(HashMap::<Method<'static>, Route>::new());
        // map.insert(method, route);
        //
        let entry = self.endpoints
            .entry(String::from(route.path))
            .or_insert(vec![]);
        entry.push(route);
    }

    pub fn include_router(&mut self, prefix: &str, router: Box<Router<'a>>) {
        // validate prefix
        if prefix.len() > 0 && !prefix.starts_with("/") {
            panic!(
                "prefix must be empty or starting with '/', got '{prefix}'"
            );
        }
        let entry = self.routers
            .entry(String::from(prefix))
            .or_insert(vec![]);
        entry.push(router);
    }

    pub fn route(&self, path: &str, request: &Request) -> Option<ResponseResult> {
        // TODO: implemented in a very stupid way, try to optimized later
        for (_path, routes) in self.endpoints.iter() {
            println!("175: matching {} with {}", path, _path);
            for route in routes.iter() {
                if route.method != request.method { continue; }
                if let Some(v) = route.execute(path, request) {
                    return Some(v);
                } else {
                    continue;
                }
            }
        }

        for (prefix, routers) in self.routers.iter() {
            println!("187: matching {} with {}", path, prefix);
            if !path.starts_with(prefix) { continue; }
            let subpath = &path[prefix.len()..];
            if subpath.len() != 0 && !subpath.starts_with("/") { continue; }

            for router in routers {
                match router.route(subpath, request) {
                    Some(resp) => return Some(resp),
                    None => continue,
                }
            }
        }
        None
    }
}

