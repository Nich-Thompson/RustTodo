#[macro_use]
extern crate yew;

use yew::html::*;

struct Model {
    input: String,
    todos: Vec<String>,
}

enum Msg {
    Add,
    Update(String),
    Remove(usize),
    RemoveAll,
    Nothing
}

fn update(_: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Add => {}
        Msg::Update(s) => {}
        Msg::Remove(i) => {}
        Msg::RemoveAll => {}
        Msg::Nothing => {}
    }
}

fn view(model: &Model) -> Html<Msg> {
    html! {
        <div>
            <div>
                <h1>{"Todo App"}</h1>
                <input />
            </div>
            <div>
                <button>{"Delete all todos"}</button>
            </div>
            <div>
                <ul>
                </ul>
            </div>
        </div>
    }
}

fn main() {
    println!("Hello, world!");
    let model = Model {
        todos: vec![],
        input: "".to_string(),
    };
    program(model, update, view);
}

