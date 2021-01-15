#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket_contrib;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use n4;
use n4::{MDContent, MenuItem};
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug)]
struct BasicPage {
    main_menu: HashMap<String, MenuItem>,
    content: MDContent,
    licensing: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentList {
    main_menu: HashMap<String, MenuItem>,
    content: Vec<n4::PageContent>,
    current_path: String,
}

fn cc_licensing() -> HashMap<String, String> {
    let list_opener = "<li class=\"list-inline-item\"><a rel=\"license\" href=\"https://creativecommons.org/licenses/";
    let scope_string = "/4.0/\" title=\"The content of this section (inside the 'main' tag), excepting the comments, is licensed under a Creative Commons ";
    let middle_first = "4.0 International License(https://creativecommons.org/licenses/";
    let middle_second = "/4.0/).  Content in comments (inside the 'commento' id) is subject to the commento.io licensing terms. All other content on the page is subject to US copyright as listed at the bottom of the page\"><img alt=\"Creative Commons License\" style=\"border-width:0\" src=\"https://i.creativecommons.org/l/";
    let list_closer = "/4.0/80x15.png\" /></a></li>";
    let mut licenses: HashMap<String, String> = HashMap::new();
    licenses.insert(
        String::from("cc-by"),
        format!(
            "{}by{}ShareAlike {}by{}by{}",
            list_opener, middle_first, scope_string, middle_second, list_closer
        ),
    );
    licenses.insert(
        String::from("cc-by-sa"),
        format!(
            "{}by-sa{}Attribution-ShareAlike {}by-sa{}by-sa{}",
            list_opener, scope_string, middle_first, middle_second, list_closer
        ),
    );
    licenses.insert(
        String::from("cc-by-nd"),
        format!(
            "{}by-nd{}Attribution-NoDerivatives {}by-nd{}by-nd{}",
            list_opener, scope_string, middle_first, middle_second, list_closer
        ),
    );
    licenses.insert(
        String::from("cc-by-nc"),
        format!(
            "{}by-nc{}Attribution-NonCommercial {}by-nc{}by-nc{}",
            list_opener, scope_string, middle_first, middle_second, list_closer
        ),
    );
    licenses.insert(
        String::from("cc-by-nc-sa"),
        format!(
            "{}by-nc-sa{}Attribution-NonCommercial-ShareAlike {}by-nc-sa{}by-nc-sa{}",
            list_opener, scope_string, middle_first, middle_second, list_closer
        ),
    );
    licenses.insert(
        String::from("cc-by-nc-nd"),
        format!(
            "{}by-nc-nd{}Attribution-NonCommercial-NoDerivatives {}by-nc-nd{}by-nc-nd{}",
            list_opener, scope_string, middle_first, middle_second, list_closer
        ),
    );
    licenses.insert(
        String::from("cc"),
        String::from(
            "<li class=\"list-inline-item\"><a rel=\"license\" href=\"/LICENSE\" title=\"All rights reserved\"><img alt=\"Copyright Statement\" style=\"border-width:0\" height=\"15px\" src=\"/static/images/copyright.svg\"></a></li>"
        )
    );
    licenses
}

fn main() {
    let menus = n4::tree_to_menus(n4::generate_content_state());

    rocket::ignite()
        .manage(menus)
        .attach(Template::fairing())
        .mount("/", routes![sitemap, robots, index, articles])
        .mount(
            "/static",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")).rank(2),
        )
        .launch();
}

// <-- Helper Routes "first"
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

#[get("/favicon.ico")]
fn favicon() -> Redirect {
    Redirect::permanent("/static/images/favicon.ico")
}
// --> End helpers

/// Front Page Route
#[get("/")]
fn index(menus: State<HashMap<String, MenuItem>>) -> Template {
    let md_files_path: &str = "/home/anon/Documents/gatewaynode_notes/website"; //TODO add to config management
    let full_content = n4::read_single_page(Path::new(&format!(
        "{}/{}",
        md_files_path, "blog/Webpack build caching errors....md"
    )));
    let context = BasicPage {
        main_menu: menus.clone(),
        content: full_content.markdown,
        licensing: cc_licensing(),
    };

    Template::render("index", context)
}

/// Everything Else Route
#[get("/<article..>", rank = 5)]
fn articles(article: PathBuf, menus: State<HashMap<String, MenuItem>>) -> Template {
    let mut content = MDContent::default();

    let md_files_path: &str = "/home/anon/Documents/gatewaynode_notes/website";
    // If Markdown file exists
    if Path::new(&format!(
        "{}/{}{}",
        md_files_path,
        article.to_string_lossy(),
        ".md"
    ))
    .exists()
    {
        let full_content = n4::read_single_page(Path::new(&format!(
            "{}/{}{}",
            md_files_path,
            article.to_string_lossy(),
            ".md"
        )));
        let context = BasicPage {
            main_menu: menus.clone(),
            content: full_content.markdown,
            licensing: cc_licensing(),
        };
        return Template::render("article", context);
    }
    // If Directory exists
    else if Path::new(&format!("{}/{}", md_files_path, article.to_string_lossy())).is_dir() {
        let content = n4::read_full_dir_sorted(
            format!("{}/{}", md_files_path, article.to_string_lossy()).as_str(),
        );
        let context = ContentList {
            main_menu: menus.clone(),
            content: content,
            current_path: article.to_string_lossy().to_string(),
        };
        return Template::render("directory", context);
    }
    // Not found
    else {
        content.title = String::from("Not Found");
        content.body = format!("This is the file router fail: {:#?}", article); // TODO send to 404 function
    }

    let context = BasicPage {
        main_menu: menus.clone(),
        content: content,
        licensing: cc_licensing(),
    };
    Template::render("article", context)
}
