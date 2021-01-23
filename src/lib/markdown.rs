use comrak::{markdown_to_html, ComrakOptions};
use serde::Serialize;
use std::{
    fs::{create_dir_all, read_to_string, write},
    io::Result,
    path::Path,
};
use tinytemplate::TinyTemplate;

#[derive(Serialize)]
struct Context {
    content: String,
}

fn render(markdown: &str, template: &str) -> String {
    let mut options = ComrakOptions::default();

    options.extension.autolink = true;
    options.extension.description_lists = true;
    options.extension.footnotes = true;
    options.extension.strikethrough = true;
    options.extension.superscript = true;
    options.extension.table = true;
    options.render.unsafe_ = true;

    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&tinytemplate::format_unescaped);
    tt.add_template("tpl", template).unwrap();

    let context = Context {
        content: markdown_to_html(markdown, &options),
    };

    tt.render("tpl", &context).unwrap()
}

fn render_and_write(input: &str, output: &str, template: &str) -> Result<()> {
    let input = Path::new(input);
    let output = Path::new(output);

    /* Read Markdown and render HTML. */
    let markdown = read_to_string(input)?;
    let html = render(markdown.as_str(), template);

    /* Write HTML to output file. */
    create_dir_all(output.parent().unwrap())?;
    write(output, html)?;

    Ok(())
}

pub fn recursive_render(
    input: &str,
    output: &str,
    template: &str,
) -> Result<()> {
    let root_input = Path::new(input);
    let root_output = Path::new(output);

    for entry in root_input.read_dir()? {
        let entry = entry?;
        let input = entry.path();
        let output = root_output.join(
            &input
                .strip_prefix(&root_input)
                .unwrap()
                .with_extension("html"),
        );

        if input.is_dir() {
            recursive_render(
                &input.to_str().unwrap(),
                &output.to_str().unwrap(),
                &template,
            )?;
        } else {
            render_and_write(
                input.to_str().unwrap(),
                output.to_str().unwrap(),
                template,
            )?;
        }
    }

    Ok(())
}
