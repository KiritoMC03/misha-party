use js_sys::Math::random;
use yew::prelude::*;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, WebSocket};

#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone();
    videos.iter().map(|video| {
        let on_video_select = {
            let on_click = on_click.clone();
            let video = video.clone();
            Callback::from(move |_| {
                on_click.emit(video.clone());
            })
        };

        html! {
            <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }).collect()
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let videos = use_state(|| vec![]);
    let selected_video = use_state(|| None);
    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video))
        })
    };

    let details = selected_video.as_ref().map(|video| html! {
        <VideoDetails video={video.clone()} />
    });

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{"Videos to watch"}</h3>
                <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
            { for details }
        </>
    }
}

fn main() {
    let income_addr = if cfg!(debug_assertions) {
        "ws://127.0.0.1:8000/income"
    } else {
        "wss://mishka-party.online/income"
    };
    let outcome_addr = if cfg!(debug_assertions) {
        "ws://127.0.0.1:8000/outcome"
    } else {
        "wss://mishka-party.online/outcome"
    };

    create_income_socket(income_addr, outcome_addr.to_string());

    yew::Renderer::<App>::new().render();
}

fn create_outcome_socket(url: &str) {
    let outcome_socket = create_socket(url);

    let outcome_socket_clone = outcome_socket.clone();
    let outcome_on_open: Closure<dyn Fn()> = Closure::new(move || {
        log(format!("outcome_on_open").as_str());
        let _ = outcome_socket_clone.send_with_str(format!("kuku epta {}", random()).as_str());
    });
    outcome_socket.set_onopen(Some(outcome_on_open.as_ref().unchecked_ref()));
    outcome_on_open.forget();// It is not good practice, just for simplification!
}

fn create_income_socket(url: &str, outcome_addr: String) {
    // let outcome_socket_clone = outcome_socket.clone();
    let on_open: Closure<dyn Fn()> = Closure::new(move || {
        log(format!("income_on_open").as_str());
        create_outcome_socket(outcome_addr.as_str());
    });
    let on_message: Closure<dyn Fn(_)> = Closure::new(move |e: MessageEvent| {
        log(format!("New Message: {:?}", e.data().as_string()).as_str());
        // let _ = send_socket_clone.send_with_str(format!("kuku epta").as_str());
    });
    let income_socket = create_socket(url);
    income_socket.set_onopen(Some(on_open.as_ref().unchecked_ref()));
    income_socket.set_onmessage(Some(on_message.as_ref().unchecked_ref()));

    on_message.forget(); // It is not good practice, just for simplification!
    on_open.forget(); // It is not good practice, just for simplification!
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn create_socket(url: &str) -> WebSocket {
    let socket = WebSocket::new(url)
        .expect("Failed to create WebSocket");
    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        console_log!("Message: {:?}", e.data().as_string().unwrap());
    });
    // set message event handler on WebSocket
    socket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    // forget the callback to keep it alive
    onmessage_callback.forget();
    socket
}