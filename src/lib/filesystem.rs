use std::{
    fs::{canonicalize, create_dir_all, write},
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
