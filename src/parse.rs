use swc_common::{Fold, Visit, VisitWith};
use swc_ecma_ast::*;

#[derive(Fold)]
struct ImportVisitor {
    entries: Vec<String>,
}

impl ImportVisitor {
    fn new() -> ImportVisitor {
        ImportVisitor { entries: Vec::new() }
    }
}

impl Visit<ImportDecl> for ImportVisitor {
    fn visit(&mut self, node: &ImportDecl) {
        self.entries.push(node.src.value.to_string());
    }
}

impl Visit<CallExpr> for ImportVisitor {
    fn visit(&mut self, node: &CallExpr) {
        match &node.callee {
            ExprOrSuper::Expr(box Expr::Ident(Ident { sym, .. })) if sym == "require" => {
                if let Some(ExprOrSpread {
                    expr: box Expr::Lit(Lit::Str(Str { value, .. })),
                    ..
                }) = node.args.first()
                {
                    self.entries.push(value.to_string());
                } else {
                    // TOOD: warn about require?
                }
            }
            _ => {
                node.visit_children(self);
            }
        }
    }
}

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

    fn parse<'a, I: Into<SourceFileInput<'a>>>(source: I) -> Result<ImportVisitor, ()> {
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

        let mut parser = Parser::new_from(Session { handler: &handler }, lexer);

        let module = parser
            .parse_module()
            .map_err(|mut err| {
                // TODO: improve error handling
                err.emit();
                ()
            })
            .expect("parser error"); // TODO: improve error handling

        let mut imports = ImportVisitor::new();

        module.visit_with(&mut imports);

        Ok(imports) // TODO: improve errors
    }

    #[test]
    fn parse_imports() {
        let source = r#"
import 'url-search-params-polyfill';
import React, /* example */ {useState} from "react";
import { render } from 'react-dom';
import * as Sentry from "@sentry/browser";

import {State} from "./app/state";
import "../example.rs"
"#;

        let sourcemap: Arc<SourceMap> = Default::default();

        let src = sourcemap.new_source_file(FileName::Custom("test.js".into()), source.into());

        let imports = parse(&*src).unwrap();

        assert_eq!(imports.entries, vec![
            "url-search-params-polyfill",
            "react",
            "react-dom",
            "@sentry/browser",
            "./app/state",
            "../example.rs"
        ]);
    }

    #[test]
    fn parse_require() {
        let source = r#"
require('url-search-params-polyfill');
const React, /* example */ {useState} = require("react");
var { render } = require('react-dom');
let Sentry = require("@sentry/browser");

if (true) {
  // require() is a lot more lax than esm imports...
}

const {State} = require("./app/state");
require("../example.rs")();

console.log("Ignore comments");
"#;

        let sourcemap: Arc<SourceMap> = Default::default();

        let src = sourcemap.new_source_file(FileName::Custom("test.js".into()), source.into());

        let imports = parse(&*src).unwrap();

        assert_eq!(imports.entries, vec![
            "url-search-params-polyfill",
            "react",
            "react-dom",
            "@sentry/browser",
            "./app/state",
            "../example.rs"
        ]);
    }
}
