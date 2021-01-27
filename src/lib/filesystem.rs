use std::{
    ffi::OsStr,
    fs::{canonicalize, create_dir_all, remove_dir, remove_file, write},
    include_str,
    io::Result,
    path::Path,
};

pub fn find_root(path: &str) -> Result<String> {
    let root = canonicalize(path).unwrap();
    let mut root = root.as_path();

    while !(root.join("src").exists() && root.join("tpl").exists()) {
        root = root
            .parent()
            .expect("Could not find root directory of an ssg project.");
    }

    Ok(String::from(root.to_str().unwrap()))
}

pub fn clean_path(input: &str, output: &str) -> Result<()> {
    let root_input = Path::new(input);
    let root_output = Path::new(output);

    for entry in root_output.read_dir()? {
        let entry = entry?;
        let output = entry.path();
        let input =
            root_input.join(&output.strip_prefix(&root_output).unwrap());

        if output.is_dir() {
            clean_path(&input.to_str().unwrap(), output.to_str().unwrap())?;
            if output.read_dir()?.next().is_none() {
                remove_dir(output)?;
            }
            continue;
        } else if output.extension().and_then(OsStr::to_str) == Some("html") {
            if input.exists() || input.with_extension("md").exists() {
                continue;
            }
        } else {
            if input.exists() {
                continue;
            }
        }

        remove_file(output)?;
    }

    Ok(())
}

pub fn create_template(path: &str) -> Result<()> {
    let path = Path::new(path);
    let mut content: &str;

    content = include_str!("template/.gitignore");
    write(path.join(".gitignore"), content)?;

    content = include_str!("template/README.md");
    write(path.join("README.md"), content)?;

    content = include_str!("template/src/index.md");
    create_dir_all(path.join("src"))?;
    write(path.join("src/index.md"), content)?;

    content = include_str!("template/src/static/style.css");
    create_dir_all(path.join("src/static"))?;
    write(path.join("src/static/style.css"), content)?;

    content = include_str!("template/tpl/template.html");
    create_dir_all(path.join("tpl"))?;
    write(path.join("tpl/template.html"), content)?;

    Ok(())
}
