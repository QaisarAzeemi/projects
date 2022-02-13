// use std::convert::Infallible;
// use std::net::SocketAddr;
// use hyper::{Body, Request, Response, Server};
// use hyper::service::{make_service_fn, service_fn};

//..................................................................................
     
     // ROUTING 

#![deny(warnings)]

//use futures_util::TryStreamExt; //stream::try_stream::
//use std::convert::Infallible;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&Method::GET, "/") => Ok(Response::new(Body::from(
            "Try POSTing data to /echo such as: `curl localhost:3000/echo -XPOST -d 'hello world'`",
        ))),

        // Simply echo the body back to the client.
        (&Method::POST, "/echo") => Ok(Response::new(req.into_body())),

        // Convert to uppercase before sending back to client using a stream.
        (&Method::POST, "/echo/uppercase") => {
            let chunk_stream = req.into_body().map_ok(|chunk| {
                chunk
                    .iter()
                    .map(|byte| byte.to_ascii_uppercase())
                    .collect::<Vec<u8>>()
            });
            Ok(Response::new(Body::wrap_stream(chunk_stream)))
           //*response.body_mut() = Body::wrap_stream(chunk_stream);
        }

        // Reverse the entire body before sending back to the client.
        //
        // Since we don't know the end yet, we can't simply stream
        // the chunks as they arrive as we did with the above uppercase endpoint.
        // So here we do `.await` on the future, waiting on concatenating the full body,
        // then afterwards the content can be reversed. Only then can we return a `Response`.
        (&Method::POST, "/echo/reversed") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;

            let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
            Ok(Response::new(Body::from(reversed_body)))
            //*response.body_mut() = reversed.into();
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 3000).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo)) });

    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}

//...................................................................................

// #![deny(warnings)]

// use std::convert::Infallible;

// use hyper::service::{make_service_fn, service_fn};
// use hyper::{Body, Request, Response, Server};

// async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
//     Ok(Response::new(Body::from("Hello World!.... by Qaisar Azeemi")))
// }

// #[tokio::main]
// pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     //pretty_env_logger::init();

//     // For every connection, we must make a `Service` to handle all
//     // incoming HTTP requests on said connection.
//     let make_svc = make_service_fn(|_conn| {
//         // This is the `Service` that will handle the connection.
//         // `service_fn` is a helper to convert a function that
//         // returns a Response into a `Service`.
//         async { Ok::<_, Infallible>(service_fn(hello)) }
//     });

//     let addr = ([127, 0, 0, 1], 3000).into();

//     let server = Server::bind(&addr).serve(make_svc);

//     println!("Listening on http://{}", addr);

//     server.await?;

//     Ok(())
// }

// fn main() {
//     println!("Hello, world!");
// }
