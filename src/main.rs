use comrak::{markdown_to_html, ComrakOptions};
use std::io::Write;
use std::{env, fs, path};

fn main() {
    /* Get the provided arguments. */
    let args: Vec<String> = env::args().collect();

    /* Check if right amount of arguments are provided. */
    if args.len() != 2 {
        println!("Command requires one argument.");
        return;
    }
    let md_path = &args[1];

    /* Read the markdown file. */
    let markdown = match fs::read_to_string(md_path) {
        Ok(content) => content,
        Err(_) => {
            println!("File '{}' not found!", &md_path);
            return;
        }
    };

    /* Construct the path to the html output file. */
    let html_path = format!(
        "{}.html",
        path::Path::new(md_path)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
    );

    /* Generate the html. */
    let html = markdown_to_html(&markdown, &ComrakOptions::default());

    /* Generate html file. */
    let mut output: fs::File = match fs::File::create(&html_path) {
        Ok(file) => file,
        Err(_) => {
            println!("File '{}' could not be created!", &html_path);
            return;
        }
    };

    /* Write html to html file. */
    match output.write(&html.as_bytes()) {
        Ok(size) => size,
        Err(_) => {
            println!("Could not write to file '{}'", &html_path);
            return;
        }
    };
}
