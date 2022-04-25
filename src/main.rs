use md::{render_post, render_tikz, warn_reference_error};
use pulldown_cmark::CodeBlockKind::Fenced;
use pulldown_cmark::CowStr::Borrowed;
use pulldown_cmark::Tag::CodeBlock;
use pulldown_cmark::{html, CowStr, Event, Parser};
use regex::Regex;
use std::collections::HashMap;
use std::io::Write;


fn delimited_by(s: &str, start: &str, end: &str) -> Option<String> {
    let start_pos = s.find(start)?;
    let end_pos = s.find(end)?;
    Some(s[start_pos + start.len()..end_pos].to_string())
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\"', "&quot;")
        .replace('\'', "&#39;")
}

fn main() {
    let mut defs = HashMap::new();
    let reference_regex = Regex::new(r#"@ref\{(?P<term>(#?\w|\s|-|_)+)\}"#).unwrap();
    let definition_regex =
        Regex::new(r#"@def\{(?P<term>(#?\w|\s|[-_])+)\s*:\s*(?P<definition>(\w|\s)+)\}"#).unwrap();

    let input = std::fs::read_to_string("input.md").unwrap();

    let event_vec: Vec<Event> = Parser::new(&input).collect();

    let parser = Parser::new(&input)
        .into_offset_iter()
        .enumerate()
        .map(|(i, (event, span))| match (i, &event) {
            (i, Event::Text(ref inner)) => {
                if let Some(previous_event) = event_vec.get(i - 1) {
                    match previous_event {
                        Event::Start(CodeBlock(Fenced(Borrowed(label)))) => {
                            let id = delimited_by(label, "{", "}").unwrap_or_else(|| panic!("Missing ID for code block with label: {label}"));
                            let title = delimited_by(label, "[", "]").unwrap_or_else(|| panic!("Missing title for code block with id {id}"));

                            let prefix = label.split(' ').next().unwrap().trim();
                            if let Event::Text(Borrowed(code)) = event {
                                if prefix.trim() == "tikz" {
                                    let svg = render_tikz(&id, code);
                                    let content = format!(
                                        r#"<figure class=tikz-diagram>{svg}<figcaption class=tikz-title>{title}</figcaption></figure>"#
                                    );

                                    defs.insert(id, content.clone());

                                    return Event::Html(CowStr::from(content));
                                } else {

                                    let code_block = format!(
                                        r#"<fieldset><legend>{title}</legend><pre><code class='language-{prefix}' data-language='{prefix}'>{}</code></pre></fieldset>"#,
                                        escape_html(code)
                                    );
                                    defs.insert(id, code_block.clone());
                                    return Event::Html(CowStr::from(code_block))
                                }
                            }
                        }
                        _ => {
                            let mut replaced = inner.to_string();

                            for capture in definition_regex.captures_iter(inner) {
                                let term = capture.name("term").unwrap().as_str();
                                let definition = capture.name("definition").unwrap().as_str();
                                defs.insert(term.to_string(), definition.to_string());
                                let html_str = format!(
                                    r#"<span class="reference" data-definition="{}">{term}</span>"#,
                                    escape_html(definition),
                                );
                                let original = capture.get(0).unwrap().as_str();
                                replaced = replaced.replace(original, &html_str);
                            }

                            for capture in reference_regex.captures_iter(inner) {
                                let term = &capture["term"];
                                if let Some(definition) = defs.get(term) {
                                    let html_str = format!(
                                        r#"<span class="reference" data-definition="{}">{term}</span>"#,
                                        escape_html(definition)
                                    );
                                    replaced = replaced.replace(&format!("@ref{{{term}}}"), &html_str);
                                } else {
                                    warn_reference_error(&input, term, &span);
                                    replaced = replaced.replace(&format!("@ref{{{term}}}"), &format!("<font color=\"red\">@ref{{{term}}}</font>"));
                                }
                            }

                            return Event::Html(CowStr::from(replaced));
                        }
                    }
                }

                event
            }
            _ => event,
        })
        .filter(|event| !matches!(
            event,
            Event::Start(CodeBlock(Fenced(Borrowed(_label))))
            | Event::End(CodeBlock(Fenced(Borrowed(_label)))))
        );

    let mut handle = std::fs::File::create("output.html").unwrap();

    let mut html_string = String::new();

    html::push_html(&mut html_string, parser);

    let post = render_post(&html_string);
    write!(handle, "{}", post).unwrap();
}
