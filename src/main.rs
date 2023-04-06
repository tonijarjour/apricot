use tree_sitter_highlight::Highlighter;
use tree_sitter_highlight::HtmlRenderer;

fn main() {

    let highlight_names = &[
        "attribute",
        "constant",
        "function.builtin",
        "function",
        "keyword",
        "operator",
        "property",
        "punctuation",
        "punctuation.bracket",
        "punctuation.delimiter",
        "string",
        "string.special",
        "tag",
        "type",
        "type.builtin",
        "variable",
        "variable.builtin",
        "variable.parameter",
    ];


    let mut highlighter = Highlighter::new();
    use tree_sitter_highlight::HighlightConfiguration;

    let mut javascript_config = HighlightConfiguration::new(
        tree_sitter_javascript::language(),
        tree_sitter_javascript::HIGHLIGHT_QUERY,
        tree_sitter_javascript::INJECTION_QUERY,
        tree_sitter_javascript::LOCALS_QUERY,
    ).unwrap();

    javascript_config.configure(highlight_names);

    let args: Vec<String> = std::env::args().collect();
    let code = std::fs::read_to_string(&args[1]).unwrap();
    let code = code.as_bytes();

    let highlights = highlighter.highlight(
        &javascript_config,
        code,
        None,
        |_| None
    ).unwrap();

    let mut html = HtmlRenderer::new();
    html.render(highlights, code, &|h| &[66]).unwrap();

    for l in html.lines() {
        println!("{}", l);
    }
}
