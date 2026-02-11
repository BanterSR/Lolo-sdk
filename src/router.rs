use axum::{
    Router,
    routing::get,
    response::Response,
    middleware::{self, Next},
    extract::Request,
};

use crate::{dispatch,sub, LoloSdkRef};
use crate::handlers::{token, v1, v2};

pub(crate) fn router(x: LoloSdkRef) ->Router {
    Router::new()
        .route("/", get(|| async { "Lolo!" }))
        .merge(sub::routes())
        .merge(dispatch::routes()) // dispatch
        .merge(v1::routes()) // v1
        .merge(v2::routes()) // v2
        .merge(token::routes()) // token api
        .layer(middleware::from_fn(axum_log))
        .with_state(x)
}

async fn axum_log(request: Request,
            next: Next,)->Response  {
    let path = request.uri().path();
    let mtod = request.method();

    tracing::debug!("request: {} {}", mtod, path);
    next.run(request).await
}