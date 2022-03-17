use yew::prelude::*;

struct Todo {
    content: Vec<String>
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Todo {
        content: vec!["Your".to_owned(), "Todos".to_owned(), "Here".to_owned()]
    });

    let onclick = {
        let state = state.clone();
        let mut old_content = state.content.clone();
        old_content.push("wow".to_owned());

        Callback::from(move |_| {
            state.set(Todo {
                content: old_content.clone()
            })
        })
    };

    html! {
        <div>
            // <input type="text" value="a"/>
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