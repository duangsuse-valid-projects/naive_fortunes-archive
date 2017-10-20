#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate serde;
extern crate serde_json;
extern crate rocket;
extern crate rand;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use std::env::args;
use std::fs::File;
use std::io::Read;
use rocket::Request;
use rocket::response::Redirect;
use rand::Rng;

lazy_static! {
    static ref FORTUNE_DATA: Vec<Fortune> = {
    let mut prog_args = args();
    prog_args.next().unwrap();
    let file_loc = prog_args.next().unwrap_or_else(
        || panic!("argument required"),
    );
    let mut file_str = String::new();
    if let Ok(mut f) = File::open(file_loc) {
        f.read_to_string(&mut file_str).unwrap();
    }
        serde_json::from_str(&file_str).unwrap()
    };
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Fortune {
    pub content: String,
    pub author: Option<String>,
    pub link: Option<String>,
}

fn get_random_idx(len: usize) -> usize {
    Rng::gen_range(&mut rand::thread_rng(), 0, len)
}

fn try_rand_author() -> String {
    if let Some(ref a) = FORTUNE_DATA[get_random_idx(FORTUNE_DATA.len())].author {
        format!("{}", a)
    } else {
        try_rand_author()
    }
}

#[get("/body")] //fetch fortune content only
fn rand_body() -> String {
    let fortune_data_len = FORTUNE_DATA.len();
    format!(
        "{}",
        &FORTUNE_DATA[get_random_idx(fortune_data_len)].content
    )
}

#[get("/author")] //fetch random author only
fn rand_author() -> String {
    try_rand_author()
}

#[get("/findfort/<author>")]
fn find_fort(author: String) -> String {
    let mut ret = String::new();
    for i in FORTUNE_DATA.iter() {
        if let Some(ref a) = i.author {
            if &author == a {
                if let Some(ref l) = i.link {
                    ret += &format!("[{}]({})\n", i.content, l);
                } else {
                    ret += &format!("{}\n", i.content);
                }
            }
        }
    }
    ret
}

#[get("/findfort")]
fn get_all() -> String {
    let mut ret = String::new();
    for i in FORTUNE_DATA.iter() {
        if let Some(ref l) = i.link {
            ret += &format!("[{}]({})\n", i.content, l);
        } else {
            ret += &format!("{}\n", i.content);
        }
    }
    ret
}

#[get("/authors")]
fn get_all_authors() -> String {
    let mut ret = String::new();
    for i in FORTUNE_DATA.iter() {
        if let Some(ref a) = i.author {
            ret += &format!("{}\n", a);
        }
    }
    ret
}

#[get("/fortune")] //get formated fortune
fn fortune() -> String {
    let fortune_data_len = FORTUNE_DATA.len();
    let fortune_data_selected = &FORTUNE_DATA[get_random_idx(fortune_data_len)];
    if let Some(ref l) = fortune_data_selected.link {
        if let Some(ref a) = fortune_data_selected.author {
            format!("[{} --{}]({})", fortune_data_selected.content, a, l)
        } else {
            format!("[{}]({})", fortune_data_selected.content, l)
        }
    } else {
        if let Some(ref a) = fortune_data_selected.author {
            format!("{} --{}", fortune_data_selected.content, a)
        } else {
            format!("{}", fortune_data_selected.content)
        }
    }
}

#[get("/")]
fn redirect() -> Redirect {
    Redirect::to("/fortune")
}

#[error(404)]
fn not_found(req: &Request) -> String {
    format!(
        "{} {}::404,这是最吼的. :frog:",
        req.method(),
        req.uri()
    )
}

fn main() {
    rocket::ignite()
        .catch(errors![not_found])
        .mount(
            "/",
            routes![
                rand_body,
                rand_author,
                fortune,
                find_fort,
                get_all,
                get_all_authors,
                redirect,
            ],
        )
        .launch();
}
