struct ServerState {
    router: pavex_runtime::routing::Router<u32>,
    application_state: ApplicationState,
}
pub struct ApplicationState {
    s0: app::HttpClient,
}
pub fn build_application_state(v0: app::Config) -> crate::ApplicationState {
    let v1 = app::http_client(v0);
    crate::ApplicationState { s0: v1 }
}
pub async fn run(
    server_builder: pavex_runtime::hyper::server::Builder<
        pavex_runtime::hyper::server::conn::AddrIncoming,
    >,
    application_state: ApplicationState,
) -> Result<(), anyhow::Error> {
    let server_state = std::sync::Arc::new(ServerState {
        router: build_router()?,
        application_state,
    });
    let make_service = pavex_runtime::hyper::service::make_service_fn(move |_| {
        let server_state = server_state.clone();
        async move {
            Ok::<
                _,
                pavex_runtime::hyper::Error,
            >(
                pavex_runtime::hyper::service::service_fn(move |request| {
                    let server_state = server_state.clone();
                    async move {
                        Ok::<
                            _,
                            pavex_runtime::hyper::Error,
                        >(route_request(request, server_state))
                    }
                }),
            )
        }
    });
    server_builder.serve(make_service).await.map_err(Into::into)
}
fn build_router() -> Result<
    pavex_runtime::routing::Router<u32>,
    pavex_runtime::routing::InsertError,
> {
    let mut router = pavex_runtime::routing::Router::new();
    router.insert("/home", 0u32)?;
    Ok(router)
}
fn route_request(
    request: pavex_runtime::http::Request<pavex_runtime::hyper::body::Body>,
    server_state: std::sync::Arc<ServerState>,
) -> pavex_runtime::http::Response<pavex_runtime::hyper::body::Body> {
    let route_id = server_state
        .router
        .at(request.uri().path())
        .expect("Failed to match incoming request path");
    match route_id.value {
        0u32 => route_handler_0(server_state.application_state.s0.clone(), request),
        _ => panic!("This is a bug, no route registered for a route id"),
    }
}
pub fn route_handler_0(
    v0: app::HttpClient,
    v1: http::request::Request<hyper::body::Body>,
) -> http::response::Response<hyper::body::Body> {
    let v2 = app::extract_path(v1);
    let v3 = app::logger();
    app::stream_file(v2, v3, v0)
}