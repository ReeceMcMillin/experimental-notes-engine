use colored::Colorize;
use askama::Template;
use std::{io::Write, process::Command, ops::Range};

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


pub fn warn_reference_error(input: &str, term: &str, span: &Range<usize>) {
    eprintln!("{}: undefined item: `{term}`", "warning".yellow().bold());

    let event_ctx = input[span.start..span.end].to_string();
    let idx = span.start + event_ctx.find(term).unwrap();
    let line = input[..idx].chars().filter(|&x| x == '\n').count() + 1;
    let term_range = (idx, idx + term.len());
    let leading_context = &input[term_range.0 - 40..term_range.0 - 5];
    let context = &input[term_range.0 - 5..term_range.1 + 1];
    let trailing_context = &input[term_range.1 + 1..term_range.1 + 40]
        .split('\n')
        .next()
        .unwrap();

    eprintln!("       {}", "|".cyan().bold());
    eprintln!(
        " {:^5} {} {}{}\x1b[0m{}",
        line.to_string().cyan().bold(),
        "|".cyan().bold(),
        leading_context,
        context.bold().yellow(),
        trailing_context
    );
    eprintln!(
        "       {} {}{}",
        "|".cyan().bold(),
        " ".repeat(leading_context.len()),
        "^".repeat(context.len()).bold().yellow()
    );
}