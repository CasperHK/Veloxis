use gloo_net::http::Request;
use leptos::prelude::*;
use shared::Greeting;
use wasm_bindgen_futures::spawn_local;

#[component]
fn App() -> impl IntoView {
    let message = RwSignal::new(String::from("Loading greeting from API..."));

    if let Some(node) = document().get_element_by_id("boot-data") {
        if let Some(raw) = node.text_content() {
            if let Ok(greeting) = serde_json::from_str::<Greeting>(&raw) {
                message.set(format!("{} ({})", greeting.message, greeting.version));
            }
        }
    }

    spawn_local(async move {
        match Request::get("/api/greeting").send().await {
            Ok(response) => match response.json::<Greeting>().await {
                Ok(greeting) => {
                    message.set(format!("{} ({})", greeting.message, greeting.version));
                }
                Err(err) => {
                    message.set(format!("Failed to parse API response: {err}"));
                }
            },
            Err(err) => {
                message.set(format!("Failed to fetch API greeting: {err}"));
            }
        }
    });

    view! {
        <main>
            <h1>"Veloxis"</h1>
            <p>{move || message.get()}</p>
        </main>
    }
}

fn main() {
    mount_to_body(App);
}
