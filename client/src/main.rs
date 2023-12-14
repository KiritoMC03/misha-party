use std::any::Any;
use std::mem;
use std::sync::Arc;
use std::thread::park_timeout;
use std::time::Duration;
use js_sys::{ArrayBuffer, Function, JsString, Promise};
use js_sys::Math::random;
use wasm_bindgen::closure::WasmClosureFnOnce;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, spawn_local};
use web_sys::{AudioBuffer, AudioContext, Blob, BlobEvent, HtmlAudioElement, MediaDevices, MediaRecorder, MediaStream, MediaStreamConstraints, MessageEvent, Url, WebSocket, window};
use yew::platform::time::sleep;

#[function_component(App)]
fn app() -> Html {
        html! {
        <>
            <button onclick={listen}>{ "Start recording" }</button>
            <h1>{ "Hello!!!!!!" }</h1>
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

fn listen(_e: MouseEvent) {
    spawn_local(async {
        let audio_enabled = &JsValue::from_bool(true);
        let mut user_media_constraints = MediaStreamConstraints::new();
        user_media_constraints.audio(audio_enabled);
        let user_media =
            window().unwrap()
                .navigator()
                .media_devices().unwrap()
                .get_user_media_with_constraints(&user_media_constraints).unwrap();
        let result = wasm_bindgen_futures::JsFuture::from(user_media).await.unwrap();
        let media_stream: MediaStream = result.unchecked_into();
        let media_recorder = MediaRecorder::new_with_media_stream(&media_stream).unwrap();
        media_recorder.start().unwrap();

        let url = web_sys::Url::new("http://127.0.0.1:8000/a.mp3").unwrap();
        let audio_elem = HtmlAudioElement::new_with_src(url.as_str()).unwrap();
        let on_data_available: Closure<dyn Fn(_)> = Closure::new(move |event: BlobEvent| {
            if !event.is_null() {
                match event.data() {
                    None => {}
                    Some(data) => {
                        let promise = data.array_buffer();
                        // let promise = data.array_buffer();
                        spawn_local(async move {
                            let result = wasm_bindgen_futures::JsFuture::from(promise).await;
                            match result {
                                Ok(_) => {
                                    // let buffer: ArrayBuffer = val.clone().into();
                                    // let ac = AudioContext::new().unwrap();
                                    // let r=  ac.decode_audio_data(&buffer).unwrap();
                                    // let acr = wasm_bindgen_futures::JsFuture::from(r).await.unwrap();
                                    // let audio_buff: AudioBuffer = acr.into();
                                    // let blob = Blob::new_with_buffer_source_sequence(&buffer).unwrap();
                                    let url = web_sys::Url::create_object_url_with_blob(&data).unwrap();
                                    log("1");
                                    let audio_elem = HtmlAudioElement::new_with_src(url.as_str()).unwrap();
                                    log("2");
                                    let play = audio_elem.play().unwrap();
                                    log("3");
                                    let _ = wasm_bindgen_futures::JsFuture::from(play).await;
                                    audio_elem.remove();
                                    log("4");
                                }
                                Err(_) => {
                                    log("empty result")
                                }
                            }
                        });
                    }
                }
            }
            else {
                log("null");
            }
        });
        media_recorder.set_ondataavailable(Some(on_data_available.as_ref().unchecked_ref()));
        on_data_available.forget(); // It is not good practice, just for simplification!
        do_magic(media_recorder);
    });
}

fn do_magic(media_recorder: MediaRecorder) {
    spawn_local(async move {
        loop {
            sleep(Duration::from_millis(100)).await;
            let _ = media_recorder.request_data();
        }
    })
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