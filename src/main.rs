use yew::prelude::*;

struct Todo {
    content: Vec<String>,
    input: String
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Todo {
        content: vec!["Your".to_owned(), "Todos".to_owned(), "Here".to_owned()],
        input: String::from("Test")
    });

    // let mut btn_val = "Test";

    let onclick = {
        let state = state.clone();
        let mut old_content = state.content.clone();
        // old_content.push("wow".to_owned());
        old_content.push(state.input.clone());

        // old_content.remove(0); // test
        // old_content.push(btn_val.to_owned());
        // state.input = "wow".to_string();

        Callback::from(move |_| {
            state.set(Todo {
                content: old_content.clone(),
                input: state.input.clone()
            })
        })
    };

    html! {
        <div>
            // <input type="text" value={ btn_val }/>
            <input type="text" 
                value={ state.input.to_string() }
                // oninput={ oninput }
            />
            <button { onclick }>{ "Click me!" }</button>
            <ul class="item-list">
                {
                    state.content.iter().map(|item| {
                        html!{<li>{ item }</li>}
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}