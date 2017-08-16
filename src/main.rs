#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde;
extern crate serde_json;
extern crate rocket;
extern crate rand;

#[macro_use]
extern crate serde_derive;

use std::env::args;
use std::fs::File;
use std::io::Read;
use rocket::State;
use rocket::Request;
use rocket::response::Redirect;
use rand::Rng;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Fortune {
    pub content: String,
    pub author: Option<String>,
    pub link: Option<String>,
}

fn get_random_idx(len: usize) -> usize {
    Rng::gen_range(&mut rand::thread_rng(), 0, len)
}

#[get("/body")] //just fetch fortune content
fn rand_body(state: State<Vec<Fortune>>) -> String {
    let data = state.to_vec();
    let data_len = data.len();
    format!("{}", &data[get_random_idx(data_len)].content)
}
#[get("/author")] //just fetch random author
fn rand_author(state: State<Vec<Fortune>>) -> String {
    let data = state.to_vec();
    let data_len = data.len();
    if let Some(ref a) = data[get_random_idx(data_len)].author {
        format!("{}", a)
    } else {
        String::new()
    }
}

#[get("/findfort/<author>")]
fn find_fort(author: String, state: State<Vec<Fortune>>) -> String {
    let data = state.to_vec();
    let mut ret = String::new();
    for i in data {
        if let Some(ref a) = i.author {
            if &author == a {
                ret += &format!("{}\n", i.content);
            }
        }
    }
    ret
}
#[get("/fortune")] //get formated fortune
fn fortune(state: State<Vec<Fortune>>) -> String {
    let data = state.to_vec();
    let data_len = data.len();
    let data_selected = &data[get_random_idx(data_len)];
    if let Some(ref l) = data_selected.link {
        if let Some(ref a) = data_selected.author {
            format!("[{} --{}]({})", data_selected.content, a, l)
        } else {
            format!("[{}]({})", data_selected.content, l)
        }
    } else {
        if let Some(ref a) = data_selected.author {
            format!("{} --{}", data_selected.content, a)
        } else {
            format!("{}", data_selected.content)
        }
    }
}

#[get("/")]
fn redirect() -> Redirect {
    Redirect::to("/fortune")
}

#[error(404)]
fn not_found(req: &Request) -> String {
    format!("{} {} ,这是最吼的. #(滑稽)", req.method(), req.uri())
}

fn main() {
    let mut prog_args = args();
    prog_args.next().unwrap();
    let file_loc = prog_args.next().unwrap_or_else(
        || panic!("argument required"),
    );
    let mut file_str = String::new();
    if let Ok(mut f) = File::open(file_loc) {
        f.read_to_string(&mut file_str).unwrap();
    }
    let deserialized: Vec<Fortune> = serde_json::from_str(&file_str).unwrap();
    rocket::ignite()
        .catch(errors![not_found])
        .mount(
            "/",
            routes![rand_body, rand_author, fortune, find_fort, redirect],
        )
        .manage(deserialized)
        .launch();
}
