use yew::prelude::*;
// use std::collections::HashMap;
// use std::io::Read;
// use std::str::FromStr; 

// struct Model {
//     value: i64
// }
struct Todo {
    // map: HashMap<String, bool>,
    content: Vec<String>
}

#[function_component(App)]
fn app() -> Html {
    // let mut todo = Todo::new().expect("db init failed");
    // let mut todo = Todo {
    //     content: Vec::new()
    // };

    let state = use_state(|| Todo {
        content: Vec::new()
    });

    let onclick = {
        // todo.content.push("wow".to_owned());
        // todo.save();
        // let mut queue: () = state.content.as_mut().unwrap();
        let state2 = state.clone();
        let mut oldContent = state2.content;
        oldContent.push("wow1".to_owned());
        oldContent.push("wow2".to_owned());
        oldContent.push("wow3".to_owned());

        let state = state.clone();
        // let mut oldContent = state.content;
        // oldContent.push("wow".to_owned());

        Callback::from(move |_| {
            state.set(Todo {
                content: Vec::new()
            })
        })
    };

    html! {
        <div>
            <button {onclick}>{ "Click me!" }</button>
            // <span>{ todo.map[""|] }</span>
            <ul class="item-list">
                { state.content.iter().collect::<Html>() }
            </ul>
        </div>
    }
}

fn main() {
    // let mut todo = Todo::new().expect("db init failed");
    yew::start_app::<App>();
}

// impl Todo {
//     fn new() -> Result<Todo, std::io::Error> {
//         let mut f = std::fs::OpenOptions::new()
//             .write(true)
//             .create(true)
//             .read(true)
//             .open("db.txt")?;
//         let mut content = String::new();
//         f.read_to_string(&mut content)?;
//         let map: HashMap<String, bool> = content
//             .lines()
//             .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
//             .map(|v| (v[0], v[1]))
//             .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
//             .collect();
//         Ok(Todo { map })
//     }

//     // Methods

//     fn insert(&mut self, key: String) {
//         self.map.insert(key, true);
//     }
    
//     // save takes ownership
//     fn save(self) -> Result<(), std::io::Error> {
//         let mut content = String::new();
//         for (k, v) in self.map {
//             let record = format!("{}\t{}\n", k, v);
//             content.push_str(&record)
//         }
//         std::fs::write("db.txt", content)
//     }

//     fn complete(&mut self, key: &String) -> Option<()> {
//         match self.map.get_mut(key) {
//             Some(v) => Some(*v = false),
//             None => None,
//         }
//     }
// }

// #[function_component(App)]
// fn app() -> Html {
//     let state = use_state(|| Model {
//         value: 0
//     });

//     let onclick = {
//         let state = state.clone();

//         Callback::from(move |_| {
//             state.set(Model {
//                 value: state.value + 1
//             })
//         })
//     };
//     let items = (1..=10).collect::<Vec<_>>();

//     html! {
//         <div>
//             <button {onclick}>{ "+1" }</button>
//             <p>{ state.value }</p>
//             <ul class="item-list">
//                 { items.iter().collect::<Html>() }
//             </ul>
//         </div>
//     }
// }