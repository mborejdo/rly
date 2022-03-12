use anyhow::{format_err, Error};
use serde_urlencoded;
use yew::services::{
    ConsoleService,
};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

pub enum Msg {
    GotInput(String),
    Clicked,
    FetchReady(Result<String, Error>),
    Ignore,
}

pub struct Model {
    link: ComponentLink<Self>,
    value: String,
    fetching: bool,
    data: String,
    ft: Option<FetchTask>
}

impl Model {
    fn fetch_data(&mut self, url: String) -> yew::services::fetch::FetchTask {
        let callback = self.link.callback(
            move |response: Response<Result<String, Error>>| {
                let (meta, body) = response.into_parts();
                if meta.status.is_success() {
                    Msg::FetchReady(body)
                } else {
                    Msg::Ignore
                }
            },
        );
        let d = vec![
            ("url".to_owned(), url.to_owned()),
        ];

        let body = serde_urlencoded::to_string(d)
            .map_err(|_| format_err!("Failed to serialize data"));

        let request = Request::post("/")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .unwrap();

        FetchService::fetch(request, callback).unwrap()
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {

        Model {
            link,
            value: "".into(),
            fetching: false,
            data: "".into(),
            ft: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotInput(new_value) => {
                self.value = new_value;
            }
            Msg::Clicked => {
                ConsoleService::log("yew log.");
                self.ft = Some(self.fetch_data(self.value.to_string()));
            }
            Msg::FetchReady(body) => {
                ConsoleService::log("fetching ...");
                self.fetching = false;
                self.data = body.unwrap();
            }
            Msg::Ignore => {
                return false;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="rly-wrapper">
                <div class="rly-index">
                    <div class="rly-title">
                        <h1>{ "RLY?!" }</h1>
                        <h4>{ "rly simple url-shortener" }</h4>
                    </div>
                    <div class="rly-input">
                        <input
                            value=&self.value
                            oninput=self.link.callback(|e: InputData| Msg::GotInput(e.value))
                            placeholder="https://rly-rly-rly-long-url.de"
                        />
                    </div>
                    <div class="rly-submit">
                        <button 
                        class="rly-btn"
                            onclick=self.link.callback(|_| Msg::Clicked)
                        >
                            { "RLY!" }
                        </button>
                    </div>
                </div>
                <div class="rly-result">
                    {&format!("{}", self.data)}
                </div>
            </div>
        }
    }
}