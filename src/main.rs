mod langs;

fn main() {
    let mut configs = std::collections::HashMap::new();

    configs.insert("javascript".to_string(), langs::javascript());
    configs.insert("typescript".to_string(), langs::typescript());
    configs.insert("tsx".to_string(), langs::tsx());
    configs.insert("html".to_string(), langs::html());
    configs.insert("css".to_string(), langs::css());
    configs.insert("json".to_string(), langs::json());
    configs.insert("rust".to_string(), langs::rust());
    configs.insert("toml".to_string(), langs::toml());
    configs.insert("c".to_string(), langs::c());

    let mut highlight_names = Vec::new();

    configs.values().for_each(|c| {
        highlight_names.extend(c.query.capture_names().iter().cloned())
    });
    configs
        .values_mut()
        .for_each(|c| c.configure(&highlight_names));

    let highlight_styles: Vec<String> = highlight_names
        .iter()
        .map(|n| format!("class=\"ts-{}\"", n.replace(".", "-")))
        .collect();

    let args: Vec<String> = std::env::args().collect();
    let code = std::fs::read_to_string(&args[1]).unwrap();
    let code = code.as_bytes();

    let language = &configs[&args[2]];

    use tree_sitter_highlight::Highlighter;
    let mut highlighter = Highlighter::new();
    let highlights = highlighter
        .highlight(&language, code, None, |l| configs.get(l))
        .unwrap();

    use tree_sitter_highlight::HtmlRenderer;
    let mut html = HtmlRenderer::new();
    html.render(highlights, code, &|h| highlight_styles[h.0].as_bytes())
        .unwrap();

    for l in html.lines() {
        print!("{}", l);
    }
}
