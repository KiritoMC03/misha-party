use yew::prelude::*;
use serde::Deserialize;
use wasm_bindgen::closure::IntoWasmClosure;
use wasm_bindgen::prelude::*;
use web_sys::{ErrorEvent, MessageEvent, WebSocket, window};

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
    // "ws://127.0.0.1:8000/echo"
    // "wss://mishka-party.online/echo"
    let send_socket = create_socket("wss://mishka-party.online/send-sound");
    let receive_socket = create_socket("wss://mishka-party.online/receive-sound");


    let send_socket_clone = send_socket.clone();
    let on_open: Closure<dyn Fn()> = Closure::new(move || {
        let _ = send_socket_clone.send_with_str(format!("kuku epta").as_str());
    });


    send_socket.set_onopen(Some(on_open.as_ref().unchecked_ref()));
    on_open.forget();// It is not good practice, just for simplification!


    let send_socket_clone = send_socket.clone();
    let on_message: Closure<dyn Fn(_)> = Closure::new(move |e: MessageEvent| {
        log(format!("New Message: {:?}", e.data().as_string()).as_str());
        let _ = send_socket_clone.send_with_str(format!("kuku epta").as_str());
    });

    receive_socket.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    on_message.forget(); // It is not good practice, just for simplification!


    yew::Renderer::<App>::new().render();
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
    let mut socket = WebSocket::new(url)
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

fn set_timeout(window: &web_sys::Window, f: &Closure<dyn Fn()>, timeout_ms: i32) -> i32 {
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            f.as_ref().unchecked_ref(),
            timeout_ms,
        )
        .expect("should register `setTimeout` OK")
}