#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket_contrib;

use std::collections::HashMap;
use std::path::PathBuf;

use n4;
use n4::{MDContent, MenuItem, PageContent};
use rocket::response::Redirect;
use rocket::State;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

// TODO This and composite page are almost identical consolidate
#[derive(Serialize, Deserialize, Debug)]
struct BasicPage {
    main_menu: HashMap<String, MenuItem>,
    content: MDContent,
    licensing: HashMap<String, String>,
}

// TODO Consolidate with basic page
#[derive(Serialize, Deserialize, Debug)]
struct CompositePage {
    main_menu: HashMap<String, MenuItem>,
    content: PageContent,
    licensing: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentList {
    main_menu: HashMap<String, MenuItem>,
    content: Vec<n4::PageContent>,
    current_path: String,
}

// TODO This probably belongs in a plugin
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
        .mount("/", routes![sitemap, robots, index, articles, testing])
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
    n4::generate_robot_food()
}

#[get("/favicon.ico")]
fn favicon() -> Redirect {
    Redirect::permanent("/static/images/favicon.ico")
}
// --> End helpers

/// Front Page Route
#[get("/")]
fn index(menus: State<HashMap<String, MenuItem>>) -> Template {
    let full_content = n4::read_single_page(String::from("Introduction.md"));
    let context = CompositePage {
        main_menu: menus.clone(),
        content: full_content,
        licensing: cc_licensing(),
    };

    Template::render("index", context)
}

/// For testing new templates
#[get("/testing")]
fn testing(menus: State<HashMap<String, MenuItem>>) -> Template {
    let full_content = n4::read_single_page(String::from("Introduction.md"));
    let context = CompositePage {
        main_menu: menus.clone(),
        content: full_content,
        licensing: cc_licensing(),
    };

    Template::render("testing", context)
}

/// Everything Else Route
///
/// This should generally be a pretty late ranking function so other functions can easily override
#[get("/<article..>", rank = 5)]
fn articles(article: PathBuf, menus: State<HashMap<String, MenuItem>>) -> Template {
    let mut content = MDContent::default();
    // First check if a matching content file exists
    if n4::does_content_exist(article.to_string_lossy().to_string()) {
        let full_content = n4::read_single_page(format!("{}{}", article.to_string_lossy(), ".md"));
        // TODO Need to pull in active path menu_meta files based on path
        let context = BasicPage {
            main_menu: menus.clone(),
            content: full_content.markdown, // TODO change with page type consolidation
            licensing: cc_licensing(),
        };
        return Template::render("article", context);
    }
    // Next check if a matching content directory exists
    else if n4::does_directory_exist(article.to_string_lossy().to_string()) {
        let content = n4::read_full_dir_sorted(article.to_string_lossy().to_string());
        // TODO Need to pull in active path menu_meta files based on path
        let context = ContentList {
            main_menu: menus.clone(),
            content: content,
            current_path: article.to_string_lossy().to_string(),
        };
        return Template::render("directory", context);
    }
    // Last consider the path not found
    else {
        content.title = String::from("Not Found");
        content.body = format!("This is the file router fail: {:#?}", article); // TODO send to 404 handler
                                                                                // Compose the data to pass to the template
        let context = BasicPage {
            main_menu: menus.clone(),
            content: content,
            licensing: cc_licensing(),
        };
        return Template::render("article", context);
    }
}
