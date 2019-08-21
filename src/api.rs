use std::io::Write;
use std::sync::{Arc, RwLock};

use gotham::helpers::http::response::create_response;
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use gotham::router::builder::*;
use gotham::router::Router;
use gotham::state::{FromState, State};
use gotham_derive::StateData;
use http::StatusCode;
use hyper::{Body, Response};
use mime::{TEXT_HTML, TEXT_PLAIN};

use crate::counter::Counter;

pub fn start_api(addr: &str, counter: Counter, metric_name: String) {
    gotham::start(addr.to_owned(), router(counter, metric_name));
}

fn router(counter: Counter, metric_name: String) -> Router {
    let state_middleware = StateMiddleware::new(counter);
    let metric_name_middleware = StateMiddleware::new(MetricName::new(metric_name));

    let pipeline = new_pipeline()
        .add(state_middleware)
        .add(metric_name_middleware)
        .build();

    let (chain, pipelines) = single_pipeline(pipeline);

    // build a router with the chain & pipeline
    build_router(chain, pipelines, |route| {
        route.get("/").to(top_page);
        route.get("/stats").to(stats);
    })
}

#[derive(Debug, Default, Clone, StateData)]
pub struct MetricName {
    name: Arc<RwLock<String>>,
}

impl MetricName {
    fn new(name: String) -> MetricName {
        MetricName {
            name: Arc::new(RwLock::new(name)),
        }
    }

    fn cloned(&self) -> String {
        self.name.as_ref().read().unwrap().clone()
    }
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

    let metric_name = {
        let wrapper = MetricName::borrow_from(&state);
        wrapper.cloned()
    };

    writeln!(&mut w, "# TYPE envoy_{} histogram", metric_name).unwrap();

    for bc in cnt.buckets() {
        writeln!(
            &mut w,
            r#"envoy_{}_bucket{{le="{}"}} {}"#,
            metric_name, bc.le, bc.sum
        )
        .unwrap();
    }
    writeln!(&mut w, "envoy_{}_sum {}", metric_name, cnt.sum()).unwrap();
    writeln!(&mut w, "envoy_{}_count {}", metric_name, cnt.count()).unwrap();

    let res = create_response(&state, StatusCode::OK, TEXT_PLAIN, w);

    (state, res)
}
