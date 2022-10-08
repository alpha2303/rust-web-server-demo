use crate::rusty_server::http::{HttpMethod, HttpRequest, HttpResponse};

use std::collections::HashMap;

#[derive(Clone)]
pub struct Router {
    routes: HashMap<(String, HttpMethod), fn(&mut HttpRequest) -> HttpResponse>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn execute(&self, request: &mut HttpRequest) -> HttpResponse {
        let func = self
            .routes
            .get(&(request.get_uri().to_string(), request.get_method().clone()))
            .map(|x| *x);

        match func {
            Some(func) => func(request),
            None => HttpResponse::new(
                404,
                None,
                format!("URI not found: {} {}", request.get_uri(), request.get_method().as_str()).as_str(),
            ),
        }
    }

    pub fn add_route(
        &mut self,
        uri: &str,
        method: HttpMethod,
        callback: fn(&mut HttpRequest) -> HttpResponse,
    ) {
        self.routes.insert(
            (uri.to_string(), method),
            callback,
        );
    }

    pub fn get_routes(&self) -> Vec<&(String, HttpMethod)> {
        self.routes.keys().collect()
    }
}
