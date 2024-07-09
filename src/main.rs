#[macro_use]
extern crate rocket;
use renderer::render_markdown;
use rocket::response::content;
use rocket::State;
use std::collections::HashMap;
use std::fs;

mod renderer;

/// Struct used to hold all mackdown pages key is the file name and value is the html
struct Pages(HashMap<String, String>);

/// Converts all markdown files from a folder to a HashMap with markdown
fn get_pages(path: &str) -> HashMap<String, String> {
    let mut pages = HashMap::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let path = path.to_str().unwrap();
        let file = fs::read_to_string(path).unwrap();

        let entry_name = entry.path();
        let entry_name = entry_name.file_name();

        pages.insert(entry_name.unwrap().to_str().unwrap().to_string(), file);
    }
    pages
}

async fn render_pages(pages: HashMap<String, String>) -> HashMap<String, String> {
    let mut rendered_pages = HashMap::new();
    let page_vec = pages.keys().cloned().collect::<Vec<String>>();

    for (key, value) in pages {
        rendered_pages.insert(key, render_markdown(value, page_vec.clone()).await);
    }
    rendered_pages
}

#[get("/<path>")]
fn find_page(path: &str, pages: &State<Pages>) -> content::RawHtml<String> {
    let mut path = path.to_string();

    // adding md if needed
    if !path.ends_with(".md") {
        path.push_str(".md");
    }

    let loaded_pages = &pages.0;

    let page = loaded_pages.get(&path);
    if page.is_none() {
        // returning index
        return index(pages);
    }
    content::RawHtml(page.unwrap().to_string())
}

#[get("/")]
fn index(pages: &State<Pages>) -> content::RawHtml<String> {
    let loaded_pages = &pages.0;

    // getting first page
    let page: Option<&String> = loaded_pages.get(loaded_pages.keys().next().unwrap());

    content::RawHtml(page.unwrap().to_string())
}

#[launch]
async fn rocket() -> _ {
    let markdown_pages = get_pages("./pages");
    let markdown_pages = render_pages(markdown_pages).await;

    rocket::build()
        .manage(Pages(markdown_pages))
        .mount("/", routes![index, find_page])
}
