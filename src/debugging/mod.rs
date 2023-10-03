use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Build, Data, Request, Rocket};
use crate::args::Args;

pub struct RequestLogger;
pub struct WebSocketConnectionLogger;

pub fn attach(mut builder: Rocket<Build>, args: &Args) -> Rocket<Build> {
    if args.log_requests() {
        builder = builder.attach(RequestLogger);
    }
    if args.log_ws_connections() {
        builder = builder.attach(WebSocketConnectionLogger)
    }

    builder
}

#[rocket::async_trait]
impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        println!("Request: {:?}", request);
    }
}

#[rocket::async_trait]
impl Fairing for WebSocketConnectionLogger {
    fn info(&self) -> Info {
        Info {
            name: "Web Socket Connection Logger",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        use tungstenite::handshake::derive_accept_key;
        use rocket::http::uncased::eq;

        let headers = req.headers();
        let is_upgrade = headers.get("Connection")
            .any(|h| h.split(',').any(|v| eq(v.trim(), "upgrade")));

        let is_ws = headers.get("Upgrade")
            .any(|h| h.split(',').any(|v| eq(v.trim(), "websocket")));

        let is_13 = headers.get_one("Sec-WebSocket-Version").map_or(false, |v| v == "13");
        let key = headers.get_one("Sec-WebSocket-Key").map(|k| derive_accept_key(k.as_bytes()));
        match key {
            Some(key) if is_upgrade && is_ws && is_13 => {
                println!("WebSocket connection success. Key: {key}");
            },
            Some(_) | None => {
                println!(
                    "WebSocket connection failed:
                Has \"Connection: upgrade\" :{}
                Has \"Upgrade: websocket\": {}
                Has \"Sec-WebSocket-Version: 13\": {}",
                         is_upgrade, is_ws, is_13);
            }
        };
    }
}