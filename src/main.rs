use comrak::{markdown_to_html, ComrakOptions};
use serde::Serialize;
use std::{ffi::OsStr, fs, path::Path, process::exit};
use tinytemplate::TinyTemplate;
use walkdir::WalkDir;

#[macro_use]
extern crate clap;
use clap::App;

#[derive(Serialize)]
struct Context {
    title: String,
    content: String,
}

fn new(name: &str) {
    let path = Path::new(name);

    /* Check if the given name is legal first. */
    if path.components().count() > 1 {
        println!(
            "Found parent directories in name, please create and move into \
            the parent directory first and try again."
        );
        exit(1);
    }
    if path.exists() {
        println!("Directory with name '{}' already exists.", name);
        exit(1);
    }

    /* Create a new directory. */
    fs::create_dir(path).expect("Failed to create a new directory.");

    /* Initialize the directory like with `ssg init`. */
    init(path.to_str().unwrap());
}

fn init(path: &str) {
    let path = Path::new(path);

    if !(path.read_dir().unwrap().next().is_none()) {
        println!("Current directory is not empty.");
        exit(1);
    }

    create_template(path.to_str().unwrap());
}

fn gen() {
    /* Find root directory. */
    let root_path = fs::canonicalize(".").unwrap();
    let mut root_path = root_path.as_path();
    while !(root_path.join("src").exists() && root_path.join("tpl").exists()) {
        root_path = root_path
            .parent()
            .expect("Could not find root directory of an ssg project.");
    }

    let src_path = root_path.join("src");
    let tpl_path = root_path.join("tpl");
    let out_path = root_path.join("out");

    /* Create output directory. */
    if !out_path.exists() {
        fs::create_dir(&out_path).expect("Failed to create 'out' directory.");
    }

    /* Copy all static files from template. */
    let static_tpl_path = tpl_path.join("static");
    let static_out_path = out_path.join("static");
    if !static_out_path.exists() {
        fs::create_dir(&static_out_path)
            .expect("Failed to create 'static' directory.");
    }
    for entry in WalkDir::new(&static_tpl_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let static_tpl_file = entry.path();
        let static_out_file =
            out_path.join(static_tpl_file.strip_prefix(&tpl_path).unwrap());

        println!("tpl file: {}", static_tpl_file.to_str().unwrap());
        println!("out file: {}", static_out_file.to_str().unwrap());

        if static_tpl_file.is_dir() {
            if !static_out_file.exists() {
                fs::create_dir(&static_out_file)
                    .expect("Failed to create a directory.");
            }
            continue;
        }

        fs::copy(static_tpl_file, static_out_file)
            .expect("Failed to copy a file.");
    }

    // /* Read the html template */
    // let html_template: &str = String::from_utf8_lossy(
    //     &fs::read(tpl_path.join("template.html")).expect("Failed to read 'template.html' file."),
    // )
    // .parse().unwrap().as_str().unwrap();
    let html_template = fs::read_to_string(tpl_path.join("template.html"))
        .expect("Failed to read 'template.html' file.");

    /* Load it in the template engine. */
    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&tinytemplate::format_unescaped);
    tt.add_template("template", &html_template).unwrap();

    /* Walk through all source files. */
    for entry in WalkDir::new(&src_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let md_path = entry.path();
        let html_path = out_path.join(md_path.strip_prefix(&src_path).unwrap());

        /* Create parallel directories for output. */
        if md_path.is_dir() && !html_path.exists() {
            fs::create_dir(&html_path).expect("Failed to create a directory.");
        }

        /* Check if path is a markdown file. */
        match md_path.extension().and_then(OsStr::to_str) {
            Some("md") => {}
            Some(_) | None => {
                continue;
            }
        };

        let html_path = html_path.with_extension("html");

        /* Turn markdown into html. */
        let markdown =
            fs::read_to_string(md_path).expect("Markdown file not found.");
        let html = markdown_to_html(&markdown, &ComrakOptions::default());

        let context = Context {
            title: "Hello, world!".to_string(),
            content: html,
        };

        let rendered = tt.render("template", &context).unwrap();
        fs::write(html_path, rendered).expect("Failed to write html file.");
    }
}

fn create_template(path: &str) {
    let path = Path::new(path);
    let tpl_gi = std::include_str!("template/.gitignore");
    let tpl_md = std::include_str!("template/src/index.md");
    let tpl_html = std::include_str!("template/tpl/template.html");
    let tpl_css = std::include_str!("template/tpl/static/style.css");

    fs::write(path.join(".gitignore"), tpl_gi)
        .expect("Failed to write '.gitignore' file.");

    fs::create_dir(path.join("src"))
        .expect("Failed to create 'src' directory.");
    fs::write(path.join("src/index.md"), tpl_md)
        .expect("Failed to write 'index.md' file.");

    fs::create_dir(path.join("tpl"))
        .expect("Failed to create 'tpl' directory.");
    fs::write(path.join("tpl/template.html"), tpl_html)
        .expect("Failed to write 'template.html' file.");

    fs::create_dir(path.join("tpl/static"))
        .expect("Failed to create 'static' directory.");
    fs::write(path.join("tpl/static/style.css"), tpl_css)
        .expect("Failed to write 'style.css' file.");
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("new", Some(matches)) => new(matches.value_of("NAME").unwrap()),
        ("init", Some(_)) => init("."),
        ("gen", Some(_)) => gen(),
        _ => {}
    }
}
