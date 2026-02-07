use axum::Router;
use axum::routing::get;
use crate::{dispatch, LoloSdkRef};

pub(crate) fn router(x: LoloSdkRef) ->Router {
    Router::new()
        .route("/", get(|| async { "Lolo!" }))
        .merge(dispatch::routes()) // dispatch
        .with_state(x)
}