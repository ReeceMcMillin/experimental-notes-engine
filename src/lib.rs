use askama::Template;
use std::{io::Write, process::Command};

#[derive(Template)]
#[template(path = "tikz.tex", escape = "none")]
struct TikzTemplate<'a> {
    code: &'a str,
}

pub fn render_tikz(id: &str, code: &str) -> String {
    let id = id.replace('#', "");
    let f = TikzTemplate { code };
    let rendered = f.render().unwrap();

    let path = format!("output/tex/{id}.tex");
    let mut outfile = std::fs::File::create(&path).unwrap();
    write!(outfile, "{}", rendered).unwrap();

    let _latex_out = Command::new("latex")
        .args([
            "-output-directory=output",
            "-interaction=nonstopmode",
            "-output-format=dvi",
            &format!("output/tex/{id}.tex"),
        ])
        .output()
        .unwrap();

    let _dvisvgm_out = Command::new("dvisvgm")
        .args([
            "--no-fonts",
            "-e",
            "-Z 2",
            "--optimize",
            &format!("output/{id}.dvi"),
            &format!("-o output/svg/{id}.svg"),
        ])
        .output()
        .unwrap();

    let _clean_up = Command::new("rm")
        .args([
            &format!("output/{id}.dvi"),
            &format!("output/{id}.log"),
            &format!("output/{id}.aux"),
        ])
        .output()
        .unwrap();

    return std::fs::read_to_string(format!("output/svg/{id}.svg")).unwrap();
}

#[derive(Template)]
#[template(path = "post.html", escape = "none")]
struct PostTemplate<'a> {
    content: &'a str,
}

pub fn render_post(content: &str) -> String {
    let f = PostTemplate { content };
    f.render().unwrap()
}