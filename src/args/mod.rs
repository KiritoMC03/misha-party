use std::env;

const LOG_REQUESTS: &str = "--log_req";
const LOG_WS_CONNECTIONS: &str = "--log_ws_cn";

#[derive(Debug, Default)]
pub struct Args {
    items: Vec<String>,
}

impl Args {
    pub fn read_env() -> Args {
        Args {
            items: env::args().collect(),
        }
    }

    pub fn has(&self, arg: &String) -> bool {
        self.items.contains(arg)
    }

    pub fn log_requests(&self) -> bool {
        self.has(&LOG_REQUESTS.to_string())
    }

    // Is WebSocket connection need
    pub fn log_ws_connections(&self) -> bool {
        self.has(&LOG_WS_CONNECTIONS.to_string())
    }
}
