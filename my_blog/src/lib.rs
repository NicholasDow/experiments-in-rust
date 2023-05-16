use wasm_bindgen::prelude::*;
use web_sys::{Document, Window, Element};
use serde::Deserialize;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    type HtmlElement;

    #[wasm_bindgen(method, getter)]
    fn inner_html(this: &HtmlElement) -> String;

    #[wasm_bindgen(method, setter)]
    fn set_inner_html(this: &HtmlElement, inner: &str);
}

#[wasm_bindgen]
pub fn run() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let json_data = r#"
        {
            "Programming": [
                {
                    "title": "Rust and WebAssembly",
                    "content": "Learn how to build a blog using Rust and WebAssembly..."
                },
                {
                    "title": "Async Programming in JavaScript",
                    "content": "Discover the power of async programming in modern JavaScript..."
                }
            ],
            "Photography": [
                {
                    "title": "Landscape Photography Tips",
                    "content": "Capture stunning landscape photos with these essential tips..."
                }
            ],
            "Travel": [
                {
                    "title": "Top 5 European Cities to Visit",
                    "content": "Explore the best European cities for your next vacation..."
                }
            ]
        }
    "#;

    let sections: serde_json::Result<BlogSections> = serde_json::from_str(json_data);

    match sections {
        Ok(sections) => {
            for (section_title, posts) in sections {
                let section_container = create_section(&document, &section_title);
                for post in posts {
                    create_post(&document, &section_container, &post.title, &post.content);
                }
            }
        }
        Err(e) => log(&format!("Error parsing JSON data: {:?}", e)),
    }
}

#[derive(Deserialize)]
struct BlogPost {
    title: String,
    content: String,
}

type BlogSections = std::collections::HashMap<String, Vec<BlogPost>>;

fn create_section(document: &Document, section_title: &str) -> Element {
    let section_container = document
        .create_element("div")
        .expect("Failed to create a section container element");

    section_container
        .set_attribute("class", "section-container")
        .expect("Failed to set class attribute for section container");

    let title = document
        .create_element("h2")
        .expect("Failed to create a section title element");

    title.set_inner_html(section_title);

    section_container.append_child(&title).unwrap();

    let main_container = document
        .get_element_by_id("main-container")
        .expect("No main container found");

    main_container
        .append_child(&section_container)
        .expect("Failed to append section container to main container");

    section_container
}

fn create_post(document: &Document, section_container: &Element, title: &str, content: &str) {
    let post_container = document
        .create_element("div")
        .expect("Failed to create a post container element");

    post_container
        .set_attribute("class", "post-container")
        .expect("Failed to set class attribute for post container");
    let post_title = document
        .create_element("h3")
        .expect("Failed to create a post title element");

    post_title.set_inner_html(title);

    let post_content = document
        .create_element("p")
        .expect("Failed to create a post content element");

    post_content.set_inner_html(content);

    post_container.append_child(&post_title).unwrap();
    post_container.append_child(&post_content).unwrap();

    section_container
        .append_child(&post_container)
        .expect("Failed to append post container to section container");
}