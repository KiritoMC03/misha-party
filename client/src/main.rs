use log::info;
use web_sys::window;
use yew::prelude::*;

enum Msg {
    AddOne,
}

struct CounterComponent {
    count: i64,
}

impl Component for CounterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.count += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="container">
                <p>{ self.count }</p>
                <button onclick={ link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    let document = window()
        .expect("Window is undefined")
        .document()
        .expect("Document in undefined");

    info!("{}", document.to_string().as_string().unwrap());

    yew::Renderer::<CounterComponent>::new().render();
}
