use std::sync::{Arc, Mutex};

use askama::Template;
use rouille::router;
use rouille::Response;

#[derive(Template)]
#[template(path = "base.html")]
struct BaseTemplate<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate<'a> {
    _parent: BaseTemplate<'a>,
}

#[derive(Template)]
#[template(path = "counter.html")]
struct CounterTemplate<'a> {
    count: i32,
    _parent: BaseTemplate<'a>,
}

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let cloned = Arc::clone(&counter);

    rouille::start_server("127.0.0.1:3000", move |request| {
        router!(request,
            (GET) (/) => {
                let t = HomeTemplate {
                    _parent: BaseTemplate {
                        title: "HTMX Examples",
                    },
                };

                Response::html(t.render().unwrap())
            },

            (GET) (/counter) => {
                let t = CounterTemplate {
                    count: *cloned.lock().unwrap(),
                    _parent: BaseTemplate {
                        title: "Counter - HTMX Examples",
                    },
                };

                Response::html(t.render().unwrap())
            },

            (POST) (/counter/{action: String}) => {
                let mut count = cloned.lock().unwrap();

                if action == "dec" {
                    *count -=1;
                } else if action == "inc" {
                    *count +=1;
                }

                Response::text(format!("{}", *count))
            },

            _ => Response::empty_404()
        )
    });
}
