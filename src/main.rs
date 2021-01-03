#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket_contrib;

use std::collections::HashMap;
use std::path::PathBuf;

use n4;
use n4::{MDContent, MenuItem};
use rocket::State;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
struct Basic {
    main_menu: HashMap<String, MenuItem>,
    content: MDContent,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentList {
    main_menu: HashMap<String, MenuItem>,
    content: Vec<n4::PageContent>,
}

fn main() {
    let menus = n4::tree_to_menus(n4::generate_content_state());

    rocket::ignite()
        .manage(menus)
        .attach(Template::fairing())
        .mount(
            "/static",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")).rank(2),
        )
        .mount("/", routes![sitemap, robots, index, articles])
        .launch();
}

#[get("/sitemap.xml")]
fn sitemap() -> Template {
    let mut context = HashMap::new();
    context.insert("sitemap_list", n4::generate_sitemap());

    Template::render("sitemap", &context)
}

#[get("/robots.txt")]
fn robots() -> String {
    String::from(
        "User-agents: *
Allow: *

Sitemap: https://gatewaynode.com/sitemap.xml",
    )
}

#[get("/")]
fn index(menus: State<HashMap<String, MenuItem>>) -> Template {
    let mut content = MDContent::default();
    let context = Basic {
        main_menu: menus.clone(),
        content: content,
    };

    Template::render("index", context)
}

#[get("/<article..>", rank = 5)]
fn articles(article: PathBuf, menus: State<HashMap<String, MenuItem>>) -> Template {
    let mut content = MDContent::default();

    let md_files_path: &str = "/home/anon/Documents/gatewaynode_notes/website";
    // If Markdown file exists
    if std::path::Path::new(&format!(
        "{}/{}{}",
        md_files_path,
        article.to_string_lossy(),
        ".md"
    ))
    .exists()
    {
        content.title = String::from("File exists");
        content.body = format!("This is the file router: exists() = {:#?}", article);
    }
    // If Directory exists
    else if std::path::Path::new(&format!("{}/{}", md_files_path, article.to_string_lossy()))
        .is_dir()
    {
        let content = n4::read_full_dir_sorted(
            format!("{}/{}", md_files_path, article.to_string_lossy()).as_str(),
        );
        let context = ContentList {
            main_menu: menus.clone(),
            content: content,
        };
        return Template::render("directory", context);
    }
    // Not found
    else {
        content.title = String::from("Not Found");
        content.body = format!("This is the file router fail: {:#?}", article); // TODO send to 404 function
    }

    let context = Basic {
        main_menu: menus.clone(),
        content: content,
    };
    Template::render("article", context)
}
