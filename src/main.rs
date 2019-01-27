use std::fs;
use std::io;

use failure::Error;
use rouille::router;
use rouille::Request;
use rouille::Response;

fn gallery_get(request: &Request) -> Response {
    request.get_param("q").ok_or()
}

fn main() -> Result<(), Error> {
    rouille::start_server("127.0.0.1:1083", |request| {
        rouille::log(&request, io::stdout(), || {
            if let Some(e) = request.remove_prefix("/e") {
                return rouille::match_assets(&e, "e");
            }

            router!(request,
                (GET)  ["/"]                    => { static_html("web/index.html")          },

                (GET)  ["/root.css"]            => { static_css ("web/root.css")            },
                (GET)  ["/lollipop.js"]         => { static_js  ("web/lollipop.js")         },
                (GET)  ["/gallery/gallery.css"] => { static_css ("web/gallery/gallery.css") },
                (GET)  ["/jquery-3.3.1.min.js"] => { static_js  ("web/jquery-3.3.1.min.js") },

                (GET)  ["/api/search"]          => { search(request)                        },

                _ => rouille::Response::empty_404()
            )
        })
    });
}

fn static_html(path: &'static str) -> Response {
    static_file("text/html", path)
}

fn static_css(path: &'static str) -> Response {
    static_file("text/css", path)
}

fn static_js(path: &'static str) -> Response {
    static_file("application/javascript", path)
}

fn static_file(content_type: &'static str, path: &'static str) -> Response {
    Response::from_file(content_type, fs::File::open(path).expect("static"))
}
