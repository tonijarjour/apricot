use std::fmt::{Display, Formatter};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let code = std::fs::read_to_string(&args[1]).unwrap();

    let code = Code::new(code);
    code.print_as_html();
}

struct Fragment {
    start: usize,
    end: usize,
    kind: String,
}

struct Code {
    content: String,
    structure: Vec<Fragment>,
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for fragment in self.structure.iter() {
            write!(
                f,
                "{}.{}: {} {}\n",
                fragment.start,
                fragment.end,
                fragment.kind,
                &self.content[fragment.start..fragment.end]
            )?;
        }

        write!(f, "")
    }
}

impl Code {
    fn new(content: String) -> Self {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_javascript::language())
            .unwrap();

        let tree = parser.parse(content.clone(), None).unwrap();
        let mut cursor = tree.walk();
        let mut structure = Vec::new();

        flatten_helper(&mut cursor, "program", &mut structure);

        Self { content, structure }
    }

    fn print_as_html(&self) {
        let mut last_byte = 0;

        for fragment in self.structure.iter() {
            let text = &self.content[fragment.start..fragment.end];
            let prior_whitespace = &self.content[last_byte..fragment.start];
            let count_new_lines = prior_whitespace.chars().fold(0, |acc, c| {
                if c == '\n' {
                    acc + 1
                } else {
                    acc
                }
            });
            let new_lines = String::from("\n".repeat(count_new_lines));
            let spaces = prior_whitespace.len() + text.len() - count_new_lines;

            if fragment.kind.contains("punctuation") {
                print!("{}{:>spaces$}", new_lines, text);
            } else {
                print!(
                    "{}<span style=\"color: --ts-{}\">{:>spaces$}</span>",
                    new_lines, fragment.kind, text
                );
            }

            last_byte = fragment.end;
        }
    }
}

fn flatten_helper(
    cursor: &mut tree_sitter::TreeCursor,
    parent_kind: &str,
    structure: &mut Vec<Fragment>,
) {
    flatten(cursor, parent_kind, structure);

    while cursor.goto_next_sibling() {
        flatten(cursor, parent_kind, structure);
    }

    cursor.goto_parent();
}

const PUNCTUATION: &str = "[${(,.;\"`)}]";
const ASSIGNMENT: &str = "=>";

fn flatten(
    cursor: &mut tree_sitter::TreeCursor,
    parent_kind: &str,
    structure: &mut Vec<Fragment>,
) {
    let kind = if PUNCTUATION.contains(cursor.node().kind()) {
        "punctuation"
    } else if ASSIGNMENT.contains(cursor.node().kind()) {
        "assignment"
    } else {
        cursor.node().kind()
    };

    if cursor.goto_first_child() {
        flatten_helper(cursor, kind, structure);
    } else {
        structure.push(Fragment {
            start: cursor.node().start_byte(),
            end: cursor.node().end_byte(),
            kind: format!("{}_{}", parent_kind, kind),
        });
    }
}
