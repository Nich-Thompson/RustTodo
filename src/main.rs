use yew::prelude::*;
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;

struct Model {
    value: i64
}

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

//     html! {
//         <div class="myDiv">
//             <button {onclick}>{ "+1" }</button>
//             <p>{ state.value }</p>
//         </div>
//     }
// }

// fn main() {
//     // println!("Hello, world!");
//     yew::start_app::<App>();
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   // Load the MongoDB connection string from an environment variable:
   let client_uri =
    "mongodb+srv://root:root@rustcluster.wujlv.mongodb.net/test?retryWrites=true&w=majority";
    //   env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!"); // this does not work for some reason
   // A Client is needed to connect to MongoDB:
   // An extra line of code to work around a DNS issue on Windows:
   let options =
      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
         .await?;
   let client = Client::with_options(options)?;
   // Print the databases in our MongoDB cluster:
   println!("Databases:");
   for name in client.list_database_names(None, None).await? {
      println!("- {}", name);
   }
   Ok(())
}