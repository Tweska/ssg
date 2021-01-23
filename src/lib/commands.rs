use super::{
    filesystem::{create_template, find_root},
    markdown::recursive_render,
};
use clap::{load_yaml, App};
use std::{
    fs::read_to_string,
    io::{Error, ErrorKind, Result},
    path::Path,
};

pub fn cli() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("init", Some(_)) => initialize()?,
        ("gen", Some(_)) => generate()?,
        _ => {}
    }

    Ok(())
}

fn initialize() -> Result<()> {
    let path = Path::new(".");

    if !(path.read_dir()?.next().is_none()) {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "The current directory is not empty!",
        ));
    }

    create_template(path.to_str().unwrap())?;

    Ok(())
}

fn generate() -> Result<()> {
    let root = find_root(".")?;
    let root = Path::new(root.as_str());

    /* Read the HTML template. */
    let template = read_to_string(&root.join("tpl/template.html"))?;

    /* Recursive render. */
    recursive_render(
        &root.join("src").to_str().unwrap(),
        &root.join("out").to_str().unwrap(),
        &template,
    )?;

    Ok(())
}
