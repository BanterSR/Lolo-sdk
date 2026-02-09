use axum::{
    Router,
    routing::get,
    response::Response,
    middleware::{self, Next},
    extract::Request,
};

use crate::{dispatch, LoloSdkRef};

pub(crate) fn router(x: LoloSdkRef) ->Router {
    Router::new()
        .route("/", get(|| async { "Lolo!" }))
        .merge(dispatch::routes()) // dispatch
        .layer(middleware::from_fn(axum_log))
        .with_state(x)
}

async  fn axum_log(request: Request,
            next: Next,)->Response  {
    let path = request.uri().path();
    let mtod = request.method();

    tracing::debug!("request: {} {}", mtod, path);
    next.run(request).await
}