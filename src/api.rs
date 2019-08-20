use std::io::Write;

use gotham::helpers::http::response::create_response;
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::single_middleware;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::{FromState, State};
use http::StatusCode;
use hyper::{Body, Response};
use mime::{TEXT_HTML, TEXT_PLAIN};

use crate::counter::Counter;

pub fn start_api(addr: &str, counter: Counter) {
    gotham::start(addr.to_owned(), router(counter));
}

fn router(counter: Counter) -> Router {
    let middleware = StateMiddleware::new(counter);

    let pipeline = single_middleware(middleware);
    let (chain, pipelines) = single_pipeline(pipeline);

    // build a router with the chain & pipeline
    build_router(chain, pipelines, |route| {
        route.get("/").to(top_page);
        route.get("/stats").to(stats);
    })
}

fn top_page(state: State) -> (State, Response<Body>) {
    let res = create_response(
        &state,
        StatusCode::OK,
        TEXT_HTML,
        include_str!("../resources/top.html"),
    );

    (state, res)
}

fn stats(state: State) -> (State, Response<Body>) {
    let mut w = Vec::<u8>::new();

    let cnt = {
        let counter = Counter::borrow_from(&state);
        counter.get_count()
    };

    writeln!(&mut w, "# TYPE envoy_dummy_time_ms histogram").unwrap();

    for bc in cnt.buckets() {
        writeln!(
            &mut w,
            r#"envoy_dummy_time_ms_bucket{{le="{}"}} {}"#,
            bc.le, bc.sum
        )
        .unwrap();
    }
    writeln!(&mut w, "envoy_dummy_time_ms_bucket_sum {}", cnt.sum()).unwrap();
    writeln!(&mut w, "envoy_dummy_time_ms_bucket_count {}", cnt.count()).unwrap();

    let res = create_response(&state, StatusCode::OK, TEXT_PLAIN, w);

    (state, res)
}
