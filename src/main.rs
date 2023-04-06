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
        "jsx" => Configuration::jsx(),
        "tsx" => Configuration::tsx(),
        "rust" => Configuration::rust(),
        "c" => Configuration::c(),
        _ => panic!(),
    };

    let mut highlighter = Highlighter::new();

    let highlights = highlighter
        .highlight(&language.config, code, None, |l| match l {
            _ => {
                println!("{}", l);
                None
            }
        })
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
            "class=\"ts-escape\"",
            "class=\"ts-keyword\"",
            "class=\"ts-string\"",
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
        let names = &["attribute", "tag"];

        let classes = &["class=\"ts-attribute\"", "class=\"ts-tag\""];

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
            "class=\"ts-comment\"",
            "class=\"ts-constant\"",
            "class=\"ts-constant-builtin\"",
            "class=\"ts-constructor\"",
            "class=\"ts-embedded\"",
            "class=\"ts-function\"",
            "class=\"ts-function-builtin\"",
            "class=\"ts-function-method\"",
            "class=\"ts-keyword\"",
            "class=\"ts-number\"",
            "class=\"ts-operator\"",
            "class=\"ts-property\"",
            "class=\"ts-punctuation-bracket\"",
            "class=\"ts-punctuation-delimiter\"",
            "class=\"ts-punctuation-special\"",
            "class=\"ts-string\"",
            "class=\"ts-string-special\"",
            "class=\"ts-variable\"",
            "class=\"ts-variable-builtin\"",
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
            "class=\"ts-keyword\"",
            "class=\"ts-punctuation-bracket\"",
            "class=\"ts-type\"",
            "class=\"ts-type-builtin\"",
            "class=\"ts-variable-parameter\"",
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
            "class=\"ts-keyword\"",
            "class=\"ts-punctuation-bracket\"",
            "class=\"ts-type\"",
            "class=\"ts-type-builtin\"",
            "class=\"ts-variable-parameter\"",
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
            "class=\"ts-attribute\"",
            "class=\"ts-comment\"",
            "class=\"ts-constant\"",
            "class=\"ts-punctuation-bracket\"",
            "class=\"ts-string\"",
            "class=\"ts-tag\"",
            "class=\"ts-tag-error\"",
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
            "class=\"ts-attribute\"",
            "class=\"ts-comment\"",
            "class=\"ts-function\"",
            "class=\"ts-keyword\"",
            "class=\"ts-number\"",
            "class=\"ts-operator\"",
            "class=\"ts-property\"",
            "class=\"ts-punctuation-delimiter\"",
            "class=\"ts-string\"",
            "class=\"ts-string-special\"",
            "class=\"ts-tag\"",
            "class=\"ts-type\"",
            "class=\"ts-variable\"",
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
            "class=\"ts-comment\"",
            "class=\"ts-constant\"",
            "class=\"ts-delimiter\"",
            "class=\"ts-function\"",
            "class=\"ts-function-special\"",
            "class=\"ts-keyword\"",
            "class=\"ts-label\"",
            "class=\"ts-number\"",
            "class=\"ts-operator\"",
            "class=\"ts-property\"",
            "class=\"ts-string\"",
            "class=\"ts-type\"",
            "class=\"ts-variable\"",
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
            "class=\"ts-attribute\"",
            "class=\"ts-comment\"",
            "class=\"ts-constant\"",
            "class=\"ts-constant-builtin\"",
            "class=\"ts-constructor\"",
            "class=\"ts-escape\"",
            "class=\"ts-function\"",
            "class=\"ts-function-macro\"",
            "class=\"ts-function-method\"",
            "class=\"ts-keyword\"",
            "class=\"ts-label\"",
            "class=\"ts-operator\"",
            "class=\"ts-property\"",
            "class=\"ts-punctuation-bracket\"",
            "class=\"ts-punctuation-delimiter\"",
            "class=\"ts-string\"",
            "class=\"ts-type\"",
            "class=\"ts-type-builtin\"",
            "class=\"ts-variable-builtin\"",
            "class=\"ts-variable-parameter\"",
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
