#[cfg(test)]
mod tests {
    use super::*;
    use difference::assert_diff;
    use std::{panic, sync::Arc};

    use swc_common::{
        errors::{ColorConfig, Handler},
        input::SourceFileInput,
        FileName, SourceMap,
    };
    use swc_ecma_codegen::Handlers;
    use swc_ecma_parser::{
        lexer::{Input, Lexer},
        EsConfig, JscTarget, Parser, Session, Syntax,
    };

    struct Noop;

    impl Handlers for Noop {}

    fn parse<'a, I: Into<SourceFileInput<'a>>>(source: I) -> Result<(), ()> {
        // TODO: configure
        let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, None);

        // TODO: configure syntax and target
        let lexer = Lexer::new(
            Session { handler: &handler },
            Syntax::Es(EsConfig {
                dynamic_import: true,
                ..Default::default()
            }),
            JscTarget::Es2017,
            source.into(),
            None, // Comments
        );

        let mut _parser = Parser::new_from(Session { handler: &handler }, lexer);

        Ok(())
    }

    #[test]
    fn parse_file() {
        let filename = "test.js";
        let source = r#"
console.log("Hello world");
"#;

        let sourcemap: Arc<SourceMap> = Default::default();

        let src = sourcemap.new_source_file(FileName::Custom(filename.into()), source.into());

        let _ = parse(&*src).unwrap();
    }
}
