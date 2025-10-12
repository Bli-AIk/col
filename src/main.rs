use crate::token::*;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{
    input::{Stream},
    prelude::*,
};
use logos::Logos;
use owo_colors::OwoColorize;
use parser::*;
use std::fs;

mod parser;
mod token;

fn main() {
    let path = "Sample.gml";
    let content = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!();
            eprintln!(
                "{} {}",
                format!("Failed to read '{}':", path).bright_red(),
                e
            );
            std::process::exit(1);
        }
    };

    lex_with_output(&content);

    let token_iter = Token::lexer(&content)
        .spanned()
        .map(|(tok, span)| match tok {
            Ok(tok) => (tok, span.into()),
            Err(_) => {
                println!("Error token encountered: {:?}", &content[span.clone()]);
                (Token::Error, span.into())
            }
        });

    let token_stream =
        Stream::from_iter(token_iter).map((0..content.len()).into(), |(t, s): (_, _)| (t, s));

    println!();
    match parser().parse(token_stream).into_result() {
        Ok(expr) => {
            let yaml_str = serde_yaml::to_string(&expr).unwrap();
            println!("{}", yaml_str);
            let debug_str = format!("{:?}", expr);
            println!("{} {}", "Parsed:".green(), colorize_brackets(&debug_str));
        }
        Err(errs) => {
            for err in errs {
                Report::build(ReportKind::Error, ((), err.span().into_range()))
                    .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
                    //.with_code(1)
                    .with_message(err.to_string())
                    .with_label(
                        Label::new(((), err.span().into_range()))
                            .with_message(err.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .eprint(Source::from(&content))
                    .unwrap();
            }
        }
    }
}


fn colorize_brackets(input: &str) -> String {
    let colors: [&dyn Fn(&str) -> String; 5] = [
        &|s| s.red().to_string(),
        &|s| s.green().to_string(),
        &|s| s.yellow().to_string(),
        &|s| s.blue().to_string(),
        &|s| s.magenta().to_string(),
    ];

    let mut chars = input.chars().peekable();
    let mut out = String::with_capacity(input.len());
    let mut depth: usize = 0;

    let mut buf = String::new();
    let flush_buf = |buf: &mut String, out: &mut String, depth: usize, colors: &[&dyn Fn(&str) -> String]| {
        if buf.is_empty() { return; }
        if depth > 0 {
            let color_fn = colors[depth % colors.len()];
            out.push_str(&color_fn(&buf));
        } else {
            out.push_str(&buf);
        }
        buf.clear();
    };

    while let Some(c) = chars.next() {
        if c == '\x1b' && chars.peek() == Some(&'[') {
            flush_buf(&mut buf, &mut out, depth, &colors);

            let mut esc = String::new();
            esc.push('\x1b');
            if let Some(br) = chars.next() { esc.push(br); }
            while let Some(&nc) = chars.peek() {
                let nc = nc;
                chars.next();
                esc.push(nc);
                if nc == 'm' { break; }
            }
            out.push_str(&esc);
            continue;
        }

        match c {
            '(' => {
                flush_buf(&mut buf, &mut out, depth, &colors);
                let color_fn = colors[depth % colors.len()];
                out.push_str(&color_fn("("));
                depth = depth.saturating_add(1);
            }
            ')' => {
                flush_buf(&mut buf, &mut out, depth, &colors);
                if depth > 0 {
                    depth -= 1;
                    let color_fn = colors[depth % colors.len()];
                    out.push_str(&color_fn(")"));
                } else {
                    out.push(')');
                }
            }
            other => {
                buf.push(other);
            }
        }
    }

    (flush_buf)(&mut buf, &mut out, depth, &colors);

    out
}
