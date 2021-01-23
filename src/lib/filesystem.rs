use std::{
    fs::{canonicalize, copy, create_dir_all, write},
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

pub fn recursive_copy(from: &str, to: &str) -> Result<()> {
    let root_from = Path::new(from);
    let root_to = Path::new(to);

    for entry in root_from.read_dir()? {
        let entry = entry?;
        let from = entry.path();
        let to = root_to.join(&from.strip_prefix(&root_from).unwrap());

        if from.is_dir() {
            recursive_copy(&from.to_str().unwrap(), &to.to_str().unwrap())?;
        } else {
            create_dir_all(to.parent().unwrap())?;
            copy(&from, &to)?;
        }
    }

    Ok(())
}

pub fn create_template(path: &str) -> Result<()> {
    let path = Path::new(path);
    let tpl_gi = include_str!("template/.gitignore");
    let tpl_md = include_str!("template/src/index.md");
    let tpl_html = include_str!("template/tpl/template.html");
    let tpl_css = include_str!("template/tpl/static/style.css");

    write(path.join(".gitignore"), tpl_gi)?;

    create_dir_all(path.join("src"))?;
    write(path.join("src/index.md"), tpl_md)?;

    create_dir_all(path.join("tpl"))?;
    write(path.join("tpl/template.html"), tpl_html)?;

    create_dir_all(path.join("tpl/static"))?;
    write(path.join("tpl/static/style.css"), tpl_css)?;

    Ok(())
}
