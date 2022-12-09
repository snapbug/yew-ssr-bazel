use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/hi")]
    Hi,
}

#[derive(Serialize, Deserialize)]
struct UuidResponse {
    uuid: Uuid,
}

#[cfg(feature = "ssr")]
async fn fetch_uuid() -> Uuid {
    // reqwest works for both non-wasm and wasm targets.
    let resp = reqwest::get("https://httpbin.org/uuid").await.unwrap();
    let uuid_resp = resp.json::<UuidResponse>().await.unwrap();

    uuid_resp.uuid
}

#[function_component]
fn Nav() -> Html {
    html! {
        <ul>
            <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
            <li><Link<Route> to={Route::Hi}>{"Hi"}</Link<Route>></li>
        </ul>
    }
}

#[function_component]
fn Content() -> HtmlResult {
    let uuid = use_prepared_state!(async move |_| -> Uuid { fetch_uuid().await }, ())
        .map_err(|e| {
            println!("error: {:#?}", e);
            e
        })?
        .unwrap();

    Ok(html! {
        <div>{"Random UUID: "}{uuid}</div>
    })
}

#[function_component]
fn Hi() -> HtmlResult {
    Ok(html! {
        <div>{"Hi"}</div>
    })
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component]
pub fn ServerApp(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <Nav />
            <Switch<Route> render={switch} />
        </Router>
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Nav />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Hi => html! {<Hi />},
        Route::Home => {
            let fallback = html! {<div>{"Loading..."}</div>};
            html! {
                <Suspense {fallback}><Content /></Suspense>
            }
        }
    }
}
