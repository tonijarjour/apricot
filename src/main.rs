use tree_sitter_highlight::HighlightConfiguration;
use tree_sitter_highlight::Highlighter;
use tree_sitter_highlight::HtmlRenderer;

fn main() {
        let args: Vec<String> = std::env::args().collect();
        let code = std::fs::read_to_string(&args[1]).unwrap();
        let code = code.as_bytes();
    
        let language = match args[2].as_str() {
            "html" => Configuration::html(),
            "css" => Configuration::css(),
            "json" => Configuration::json(),
            "javascript" => Configuration::javascript(),
            "typescript" => Configuration::typescript(),
            "rust" => Configuration::rust(),
            "c" => Configuration::c(),
            _ => panic!(),
        };
    
        let mut highlighter = Highlighter::new();
        let highlights = highlighter
            .highlight(&language.config, code, None, |_| None)
            .unwrap();
    
        let mut html = HtmlRenderer::new();
        html.render(highlights, code, &|h| language.classes[h.0].as_bytes())
            .unwrap();
    
        for l in html.lines() {
            print!("{}", l);
        }
}

struct Configuration {
    classes: &'static [&'static str],
    config: HighlightConfiguration,
}

impl Configuration {
    fn json() -> Self {
        let names = &["escape", "keyword", "string"];

        let classes = &[
            "class=\"ts-json-escape\"",
            "class=\"ts-json-keyword\"",
            "class=\"ts-json-string\"",
        ];

        let mut config = HighlightConfiguration::new(
            tree_sitter_json::language(),
            tree_sitter_json::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap();
        config.configure(names);
        Self { classes, config }
    }

    fn jsx() -> Self {
        let names = &["tag", "attribute"];

        let classes = &["class=\"ts-jsx-tag\"", "class=\"ts-jsx-attribute\""];

        let mut config = HighlightConfiguration::new(
            tree_sitter_javascript::language(),
            tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
            tree_sitter_javascript::INJECTION_QUERY,
            tree_sitter_javascript::LOCALS_QUERY,
        )
        .unwrap();
        config.configure(names);
        Self { classes, config }
    }

    fn javascript() -> Self {
        let names = &[
            "comment",
            "constant",
            "constant.builtin",
            "constructor",
            "embedded",
            "function",
            "function.builtin",
            "function.method",
            "keyword",
            "number",
            "operator",
            "property",
            "punctuation.bracket",
            "punctuation.delimiter",
            "punctuation.special",
            "string",
            "string.special",
            "variable",
            "variable.builtin",
        ];

        let classes = &[
            "class=\"ts-js-comment\"",
            "class=\"ts-js-constant\"",
            "class=\"ts-js-constant-builtin\"",
            "class=\"ts-js-constructor\"",
            "class=\"ts-js-embedded\"",
            "class=\"ts-js-function\"",
            "class=\"ts-js-function-builtin\"",
            "class=\"ts-js-function-method\"",
            "class=\"ts-js-keyword\"",
            "class=\"ts-js-number\"",
            "class=\"ts-js-operator\"",
            "class=\"ts-js-property\"",
            "class=\"ts-js-punctuation-bracket\"",
            "class=\"ts-js-punctuation-delimiter\"",
            "class=\"ts-js-punctuation-special\"",
            "class=\"ts-js-string\"",
            "class=\"ts-js-string-special\"",
            "class=\"ts-js-variable\"",
            "class=\"ts-js-variable-builtin\"",
        ];

        let mut config = HighlightConfiguration::new(
            tree_sitter_javascript::language(),
            tree_sitter_javascript::HIGHLIGHT_QUERY,
            tree_sitter_javascript::INJECTION_QUERY,
            tree_sitter_javascript::LOCALS_QUERY,
        )
        .unwrap();
        config.configure(names);
        Self { classes, config }
    }

    fn tsx() -> Self {
        let names = &[
            "keyword",
            "punctuation.bracket",
            "type",
            "type.builtin",
            "variable.parameter",
        ];

        let classes = &[
            "class=\"keyword\"",
            "class=\"punctuation.bracket\"",
            "class=\"type\"",
            "class=\"type.builtin\"",
            "class=\"variable.parameter\"",
        ];

        let mut config = HighlightConfiguration::new(
            tree_sitter_typescript::language_tsx(),
            tree_sitter_typescript::HIGHLIGHT_QUERY,
            "",
            tree_sitter_typescript::LOCALS_QUERY,
        )
        .unwrap();
        config.configure(names);
        Self { classes, config }
    }

    fn typescript() -> Self {
        let names = &[
            "keyword",
            "punctuation.bracket",
            "type",
            "type.builtin",
            "variable.parameter",
        ];

        let classes = &[
            "class=\"keyword\"",
            "class=\"punctuation.bracket\"",
            "class=\"type\"",
            "class=\"type.builtin\"",
            "class=\"variable.parameter\"",
        ];

        let mut config = HighlightConfiguration::new(
            tree_sitter_typescript::language_typescript(),
            tree_sitter_typescript::HIGHLIGHT_QUERY,
            "",
            tree_sitter_typescript::LOCALS_QUERY,
        )
        .unwrap();
        config.configure(names);
        Self { classes, config }
    }

    fn html() -> Self {
        let names = &[
            "attribute",
            "comment",
            "constant",
            "punctuation.bracket",
            "string",
            "tag",
            "tag.error",
        ];

        let classes = &[
            "class=\"ts-html-attribute\"",
            "class=\"ts-html-comment\"",
            "class=\"ts-html-constant\"",
            "class=\"ts-html-punctuation-bracket\"",
            "class=\"ts-html-string\"",
            "class=\"ts-html-tag\"",
            "class=\"ts-html-tag-error\"",
        ];

        let mut config = HighlightConfiguration::new(
            tree_sitter_html::language(),
            tree_sitter_html::HIGHLIGHT_QUERY,
            tree_sitter_html::INJECTION_QUERY,
            "",
        )
        .unwrap();
        config.configure(names);
        Self { classes, config }
    }

    fn css() -> Self {
        let names = &[
            "attribute",
            "comment",
            "function",
            "keyword",
            "number",
            "operator",
            "property",
            "punctuation.delimiter",
            "string",
            "string.special",
            "tag",
            "type",
            "variable",
        ];

        let classes = &[
            "class=\"ts-css-attribute\"",
            "class=\"ts-css-comment\"",
            "class=\"ts-css-function\"",
            "class=\"ts-css-keyword\"",
            "class=\"ts-css-number\"",
            "class=\"ts-css-operator\"",
            "class=\"ts-css-property\"",
            "class=\"ts-css-punctuation-delimiter\"",
            "class=\"ts-css-string\"",
            "class=\"ts-css-string-special\"",
            "class=\"ts-css-tag\"",
            "class=\"ts-css-type\"",
            "class=\"ts-css-variable\"",
        ];

        let mut config = HighlightConfiguration::new(
            tree_sitter_css::language(),
            tree_sitter_css::HIGHLIGHTS_QUERY,
            "",
            "",
        )
        .unwrap();
        config.configure(names);
        Self { classes, config }
    }

    fn c() -> Self {
        let names = &[
            "comment",
            "constant",
            "delimiter",
            "function",
            "function.special",
            "keyword",
            "label",
            "number",
            "operator",
            "property",
            "string",
            "type",
            "variable",
        ];

        let classes = &[
            "class=\"ts-c-comment\"",
            "class=\"ts-c-constant\"",
            "class=\"ts-c-delimiter\"",
            "class=\"ts-c-function\"",
            "class=\"ts-c-function-special\"",
            "class=\"ts-c-keyword\"",
            "class=\"ts-c-label\"",
            "class=\"ts-c-number\"",
            "class=\"ts-c-operator\"",
            "class=\"ts-c-property\"",
            "class=\"ts-c-string\"",
            "class=\"ts-c-type\"",
            "class=\"ts-c-variable\"",
        ];

        let mut config = HighlightConfiguration::new(
            tree_sitter_c::language(),
            tree_sitter_c::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap();
        config.configure(names);
        Self { classes, config }
    }

    fn rust() -> Self {
        let names = &[
            "attribute",
            "comment",
            "constant",
            "constant.builtin",
            "constructor",
            "escape",
            "function",
            "function.macro",
            "function.method",
            "keyword",
            "label",
            "operator",
            "property",
            "punctuation.bracket",
            "punctuation.delimiter",
            "string",
            "type",
            "type.builtin",
            "variable.builtin",
            "variable.parameter",
        ];

        let classes = &[
            "class=\"ts-rs-attribute\"",
            "class=\"ts-rs-comment\"",
            "class=\"ts-rs-constant\"",
            "class=\"ts-rs-constant-builtin\"",
            "class=\"ts-rs-constructor\"",
            "class=\"ts-rs-escape\"",
            "class=\"ts-rs-function\"",
            "class=\"ts-rs-function-macro\"",
            "class=\"ts-rs-function-method\"",
            "class=\"ts-rs-keyword\"",
            "class=\"ts-rs-label\"",
            "class=\"ts-rs-operator\"",
            "class=\"ts-rs-property\"",
            "class=\"ts-rs-punctuation-bracket\"",
            "class=\"ts-rs-punctuation-delimiter\"",
            "class=\"ts-rs-string\"",
            "class=\"ts-rs-type\"",
            "class=\"ts-rs-type-builtin\"",
            "class=\"ts-rs-variable-builtin\"",
            "class=\"ts-rs-variable-parameter\"",
        ];

        let mut config = HighlightConfiguration::new(
            tree_sitter_rust::language(),
            tree_sitter_rust::HIGHLIGHT_QUERY,
            "",
            "",
        )
        .unwrap();
        config.configure(names);
        Self { classes, config }
    }
}
