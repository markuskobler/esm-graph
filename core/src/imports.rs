use std::{fs::File, io::Read, path::PathBuf, result::Result, sync::Arc};
use swc_common::{
    errors::{ColorConfig, Handler},
    input::SourceFileInput,
    FileName, Fold, SourceMap, Visit, VisitWith,
};
use swc_ecma_ast::*;
use swc_ecma_codegen::Handlers;
use swc_ecma_parser::{
    lexer::{Input, Lexer},
    EsConfig, JscTarget, Session, Syntax,
};

use crate::error::Error;

pub struct Parser {
    sourcemap: Arc<SourceMap>,
}

impl Default for Parser {
    fn default() -> Self {
        Parser {
            sourcemap: Default::default(),
        }
    }
}

impl Parser {
    pub fn parse(&self, p: PathBuf) -> Result<Vec<String>, Error> {
        let mut file = File::open(&p)?;
        let mut src = String::new();
        file.read_to_string(&mut src)?;

        let mut visitor = ImportVisitor::new();

        swc_common::GLOBALS.set(&swc_common::Globals::new(), || {
            let source = self.sourcemap.new_source_file(FileName::Real(p), src);

            // TODO: configure
            let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, None);

            // TODO: configure syntax and target
            let lexer = Lexer::<'_, SourceFileInput>::new(
                Session { handler: &handler },
                Syntax::Es(EsConfig {
                    dynamic_import: true,
                    ..Default::default()
                }),
                JscTarget::Es2017,
                SourceFileInput::from(&*source),
                None, // Comments
            );

            let mut parser = swc_ecma_parser::Parser::new_from(Session { handler: &handler }, lexer);

            let module = parser
                .parse_module()
                .map_err(|mut err| {
                    // TODO: improve error handling
                    err.emit();
                    ()
                })
                .expect("parser error"); // TODO: improve error handling

            module.visit_with(&mut visitor);
        });

        Ok(visitor.static_imports)
    }
}

#[derive(Fold)]
struct ImportVisitor {
    static_imports:  Vec<String>,
    dynamic_imports: Vec<String>,
}

impl ImportVisitor {
    fn new() -> ImportVisitor {
        ImportVisitor {
            static_imports:  Vec::new(),
            dynamic_imports: Vec::new(),
        }
    }
}

impl Visit<ImportDecl> for ImportVisitor {
    fn visit(&mut self, node: &ImportDecl) {
        self.static_imports.push(node.src.value.to_string());
    }
}

impl Visit<CallExpr> for ImportVisitor {
    fn visit(&mut self, node: &CallExpr) {
        // TODO: do we need to check for umd modules and require polyfils?
        match &node.callee {
            ExprOrSuper::Expr(box Expr::Ident(Ident { sym, .. })) if sym == "require" || sym == "import" => {
                if let Some(ExprOrSpread {
                    expr: box Expr::Lit(Lit::Str(Str { value, .. })),
                    ..
                }) = node.args.first()
                {
                    if sym == "import" {
                        // TODO: not sure if this is correct
                        self.dynamic_imports.push(value.to_string());
                    } else {
                        self.static_imports.push(value.to_string());
                    }
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

struct Noop;

impl Handlers for Noop {}

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
import React, /* example */ {useState, useContext} from "react";
import { render as reactRender } from 'react-dom';
import * as Sentry from "@sentry/browser";
import {State} from "./app/state";
import "../example.rs"

const promise = import("./dynamic/import1.js");

import("./dynamic/import2.js").then(module => {
  console.log("Loaded", module);
})
.catch(err => {
  console.log("Failed", err);
});
"#;

        let sourcemap: Arc<SourceMap> = Default::default();

        let src = sourcemap.new_source_file(FileName::Custom("test.js".into()), source.into());

        let imports = parse(&*src).unwrap();

        assert_eq!(imports.static_imports, vec![
            "url-search-params-polyfill",
            "react",
            "react-dom",
            "@sentry/browser",
            "./app/state",
            "../example.rs"
        ]);

        assert_eq!(imports.dynamic_imports, vec![
            "./dynamic/import1.js",
            "./dynamic/import2.js",
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

        assert_eq!(imports.static_imports, vec![
            "url-search-params-polyfill",
            "react",
            "react-dom",
            "@sentry/browser",
            "./app/state",
            "../example.rs"
        ]);
    }
}
