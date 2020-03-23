use crate::worker;
use serde_derive::{Deserialize, Serialize};
use yew::prelude::*;
use yew::services::ConsoleService;

pub struct App {
    link: ComponentLink<Self>,
    workers: Vec<Box<dyn Bridge<worker::Worker>>>,
    console: ConsoleService,
}

#[derive(Serialize, Deserialize)]
pub struct State {}

#[derive(Serialize, Deserialize)]
struct Entry {
    description: String,
    completed: bool,
    editing: bool,
}

pub enum Msg {
    LaunchWorker,
    DoComputations,
    ResponseHandler,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            workers: vec![],
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::LaunchWorker => {
                self.console.log("Launching a new worker!");
                let callback = self.link.callback(|_| Msg::ResponseHandler);
                let worker = worker::Worker::bridge(callback);
                self.workers.push(worker);
            }
            Msg::DoComputations => {
                self.console.log("Performing computations on the workers");
                self.workers
                    .iter_mut()
                    .for_each(|w| w.send(worker::Request::GetDataFromServer));
            }
            Msg::ResponseHandler => {
                self.console.log("Received response from worker");
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="button_container">

                    <br></br>
                    <button onclick=self.link.callback(|_| Msg::LaunchWorker)>{ "Launch a new worker" }</button>
                    <button onclick=self.link.callback(|_| Msg::DoComputations)>{ "Compute things on workers!" }</button>
                    <div>
                    <br></br>
                    <br></br>
                    <br></br>
                    <br></br>
                    {"Protip: open the console"}
                    </div>
            </div>
        }
    }
}

impl App {}
