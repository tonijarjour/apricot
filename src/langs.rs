use tree_sitter_highlight::HighlightConfiguration;

pub fn javascript() -> HighlightConfiguration {
    HighlightConfiguration::new(
        tree_sitter_javascript::language(),
        &format!(
            "{}\n{}",
            tree_sitter_javascript::HIGHLIGHT_QUERY,
            tree_sitter_javascript::JSX_HIGHLIGHT_QUERY
        ),
        tree_sitter_javascript::INJECTION_QUERY,
        tree_sitter_javascript::LOCALS_QUERY,
    )
    .unwrap()
}

pub fn typescript() -> HighlightConfiguration {
    HighlightConfiguration::new(
        tree_sitter_typescript::language_typescript(),
        &format!(
            "{}\n{}",
            tree_sitter_javascript::HIGHLIGHT_QUERY,
            tree_sitter_typescript::HIGHLIGHT_QUERY
        ),
        tree_sitter_javascript::INJECTION_QUERY,
        &format!(
            "{}\n{}",
            tree_sitter_javascript::LOCALS_QUERY,
            tree_sitter_typescript::LOCALS_QUERY
        ),
    )
    .unwrap()
}

pub fn tsx() -> HighlightConfiguration {
    HighlightConfiguration::new(
        tree_sitter_typescript::language_tsx(),
        &format!(
            "{}\n{}\n{}",
            tree_sitter_javascript::HIGHLIGHT_QUERY,
            tree_sitter_typescript::HIGHLIGHT_QUERY,
            tree_sitter_javascript::JSX_HIGHLIGHT_QUERY
        ),
        tree_sitter_javascript::INJECTION_QUERY,
        &format!(
            "{}\n{}",
            tree_sitter_javascript::LOCALS_QUERY,
            tree_sitter_typescript::LOCALS_QUERY
        ),
    )
    .unwrap()
}

pub fn html() -> HighlightConfiguration {
    HighlightConfiguration::new(
        tree_sitter_html::language(),
        tree_sitter_html::HIGHLIGHT_QUERY,
        tree_sitter_html::INJECTION_QUERY,
        "",
    )
    .unwrap()
}

pub fn css() -> HighlightConfiguration {
    HighlightConfiguration::new(
        tree_sitter_css::language(),
        tree_sitter_css::HIGHLIGHTS_QUERY,
        "",
        "",
    )
    .unwrap()
}

pub fn json() -> HighlightConfiguration {
    HighlightConfiguration::new(
        tree_sitter_json::language(),
        tree_sitter_json::HIGHLIGHT_QUERY,
        "",
        "",
    )
    .unwrap()
}

pub fn rust() -> HighlightConfiguration {
    HighlightConfiguration::new(
        tree_sitter_rust::language(),
        tree_sitter_rust::HIGHLIGHT_QUERY,
        "",
        "",
    )
    .unwrap()
}

pub fn toml() -> HighlightConfiguration {
    HighlightConfiguration::new(
        tree_sitter_toml::language(),
        tree_sitter_toml::HIGHLIGHT_QUERY,
        "",
        "",
    )
    .unwrap()
}

pub fn c() -> HighlightConfiguration {
    HighlightConfiguration::new(
        tree_sitter_c::language(),
        tree_sitter_c::HIGHLIGHT_QUERY,
        "",
        "",
    )
    .unwrap()
}
