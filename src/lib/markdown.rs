use chrono::prelude::{DateTime, Datelike, Timelike, Utc};
use comrak::{markdown_to_html, ComrakOptions};
use serde::Serialize;
use std::{
    ffi::OsStr,
    fs::{copy, create_dir_all, read_to_string, write},
    io::Result,
    path::Path,
};
use tinytemplate::TinyTemplate;
use yaml_rust::YamlLoader;

#[derive(Serialize)]
struct Context {
    content: String,
    meta: Meta,
    options: Options,
}

#[derive(Serialize)]
struct Meta {
    title: Option<String>,
    author: Option<String>,
    description: Option<String>,
    language: Option<String>,
    generation_date: String,
    generation_time: String,
    source: String,
}

#[derive(Serialize)]
struct Options {
    publish: bool,
}

fn split_meta_and_content(markdown: &str) -> (String, String) {
    let components: Vec<&str> = markdown.splitn(3, "---").collect();

    if components[0] == "" && components.len() == 3 {
        return (String::from(components[1]), String::from(components[2]));
    }

    (String::from(""), String::from(markdown))
}

fn as_string(s: &str) -> Option<String> {
    Some(String::from(s))
}

fn parse_yaml(yaml: &str, filename: &str) -> (Meta, Options) {
    let filename = format!("{}", filename);
    let dt: DateTime<Utc> = Utc::now();
    let date = format!("{:02}-{:02}-{:02}", dt.day(), dt.month(), dt.year());
    let time =
        format!("{:02}:{:02}:{:02}", dt.hour(), dt.minute(), dt.second());

    if yaml == "" {
        return (
            Meta {
                title: None,
                author: None,
                description: None,
                language: None,
                generation_date: date,
                generation_time: time,
                source: filename,
            },
            Options { publish: true },
        );
    }

    let yaml = YamlLoader::load_from_str(yaml).unwrap();
    let yaml = &yaml[0];

    let title = &yaml["title"].as_str().and_then(as_string);
    let author = &yaml["author"].as_str().and_then(as_string);
    let description = &yaml["description"].as_str().and_then(as_string);
    let language = &yaml["language"].as_str().and_then(as_string);

    let publish = match &yaml["publish"].as_bool() {
        Some(b) => b.clone(),
        None => true,
    };

    (
        Meta {
            title: title.clone(),
            author: author.clone(),
            description: description.clone(),
            language: language.clone(),
            generation_date: date,
            generation_time: time,
            source: filename,
        },
        Options { publish: publish },
    )
}

fn render(markdown: &str, template: &str, filename: &str) -> Option<String> {
    let mut md_options = ComrakOptions::default();

    md_options.extension.autolink = true;
    md_options.extension.description_lists = true;
    md_options.extension.footnotes = true;
    md_options.extension.strikethrough = true;
    md_options.extension.superscript = true;
    md_options.extension.table = true;
    md_options.render.unsafe_ = true;

    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&tinytemplate::format_unescaped);
    tt.add_template("tpl", template).unwrap();

    /* Extract YAML from file. */
    let (yaml, markdown) = split_meta_and_content(markdown);
    let (meta, options) = parse_yaml(yaml.as_str(), filename);

    if !options.publish {
        return None;
    }

    let context = Context {
        content: markdown_to_html(markdown.as_str(), &md_options),
        meta: meta,
        options: options,
    };

    Some(tt.render("tpl", &context).unwrap())
}

fn render_and_write(input: &str, output: &str, template: &str) -> Result<()> {
    let input = Path::new(input);
    let output = Path::new(output);

    /* Read Markdown and render HTML. */
    let markdown = read_to_string(input)?;
    let html = match render(
        markdown.as_str(),
        template,
        input.file_name().unwrap().to_str().unwrap(),
    ) {
        Some(html) => html,
        None => return Ok(()),
    };

    /* Write HTML to output file. */
    create_dir_all(output.parent().unwrap())?;
    write(output, html)?;

    Ok(())
}

pub fn recursive_render(
    input: &str,
    output: &str,
    template: &str,
    ignore_unchanged: bool,
) -> Result<()> {
    let root_input = Path::new(input);
    let root_output = Path::new(output);

    for entry in root_input.read_dir()? {
        let entry = entry?;
        let input = entry.path();
        let mut output =
            root_output.join(&input.strip_prefix(&root_input).unwrap());

        if input.is_dir() {
            recursive_render(
                &input.to_str().unwrap(),
                &output.to_str().unwrap(),
                &template,
                ignore_unchanged,
            )?;
            continue;
        }

        if input.extension().and_then(OsStr::to_str) == Some("md") {
            output = output.with_extension("html");
        }

        if ignore_unchanged
            && output.exists()
            && output.metadata()?.modified()? > input.metadata()?.modified()?
        {
            continue;
        }

        if input.extension().and_then(OsStr::to_str) == Some("md") {
            render_and_write(
                input.to_str().unwrap(),
                output.to_str().unwrap(),
                template,
            )?;
        } else {
            create_dir_all(output.parent().unwrap())?;
            copy(input, output)?;
        }
    }

    Ok(())
}
