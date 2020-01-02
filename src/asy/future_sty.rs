use {
    futures::{
        // Extension trait for futures 0.1 futures, adding the `.compat()` method
        // which allows us to use `.await` on 0.1 futures.
        compat::Future01CompatExt,
        // Extension traits providing additional methods on futures.
        // `FutureExt` adds methods that work for all futures, whereas
        // `TryFutureExt` adds methods to futures that return `Result` types.
        future::{FutureExt, TryFutureExt},
    },
    hyper::{
        // A function which runs a future to completion using the Hyper runtime.
        rt::run,
        // This function turns a closure which returns a future into an
        // implementation of the the Hyper `Service` trait, which is an
        // asynchronous function from a generic `Request` to a `Response`.
        service::service_fn,

        // Miscellaneous types from Hyper for working with HTTP.
        Body,
        Client,
        Request,
        Response,
        Server,
        Uri,
    },
    std::net::SocketAddr,
};

async fn serve_req(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("hello, world!")))
}

async fn run_server(addr: SocketAddr) {
    println!("Listening on http://{}", addr);

    let serve_future =
        Server::bind(&addr).serve(|| service_fn(|req| serve_req(req).boxed().compat()));
    if let Err(e) = serve_future.compat().await {
        eprintln!("server error: {}", e);
    }
}

pub fn run_test() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let future_03_future = run_server(addr);
    let future_01_future = future_03_future.unit_error().boxed().compat();
    run(future_01_future);
}
