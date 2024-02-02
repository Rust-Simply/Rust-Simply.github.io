#![allow(unused)]
#![allow(non_snake_case)]

use rust_simply_website::App;

use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    pre_cache_static_routes_with_props(
        &ServeConfigBuilder::new_with_router(dioxus_fullstack::router::FullstackRouterConfig::<
            Route,
        >::default())
        .assets_path("docs")
        .incremental(IncrementalRendererConfig::default().static_dir("docs"))
        .build(),
    )
    .await
    .unwrap();
}
#[cfg(feature = "web")]
fn main() {
    dioxus_web::launch_with_props(
        dioxus_fullstack::router::RouteWithCfg::<Route>,
        dioxus_fullstack::prelude::get_root_props_from_document()
            .expect("Failed to get root props from document"),
        dioxus_web::Config::default().hydrate(true),
    );
}

#[cfg(feature = "csr")]
fn main() {
    dioxus_web::launch(Routes);
}

#[cfg(not(any(feature = "web", feature = "ssr", feature = "csr")))]
fn main() {}


fn Routes(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

#[derive(Clone, Debug, PartialEq, Routable, Serialize, Deserialize)]
enum Route {
    #[route("/")]
    App {},
}
