use crate::parser::*;
use crate::token::*;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{input::Stream, prelude::*};
use logos::Logos;

/// Handle parsing operations
pub struct ParseHandler;

impl ParseHandler {
    /// Perform lexical analysis and display tokens
    pub fn perform_lexical_analysis(content: &str) {
        lex_with_output(content);
    }

    /// Parse source code and return AST
    pub fn parse_source_code(content: &str) -> Result<program::Program, ()> {
        let token_iter = Token::lexer(content)
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
        match program_parser().parse(token_stream).into_result() {
            Ok(program) => {
                crate::output_handler::OutputHandler::display_ast(&program);
                Ok(program)
            }
            Err(errs) => {
                Self::display_parse_errors(errs, content);
                Err(())
            }
        }
    }

    /// Display parsing errors
    fn display_parse_errors(errors: Vec<Rich<Token>>, content: &str) {
        for err in errors {
            Report::build(ReportKind::Error, ((), err.span().into_range()))
                .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
                .with_message(err.to_string())
                .with_label(
                    Label::new(((), err.span().into_range()))
                        .with_message(err.reason().to_string())
                        .with_color(Color::Red),
                )
                .finish()
                .eprint(Source::from(content))
                .unwrap();
        }
    }
}
