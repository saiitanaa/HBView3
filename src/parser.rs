use std::path::Path;

use tree_sitter::{Language, Parser};

pub struct SourceAnalysis {
    pub path: String,
    pub functions: Vec<String>,
    pub calls: Vec<String>,
}

pub fn parse_file(path: &Path) -> Result<SourceAnalysis, String> {
    let source = std::fs::read_to_string(path)
        .map_err(|error| format!("Failed to read {}: {error}", path.display()))?;

    let language = language_for_path(path)?;

    let mut parser = Parser::new();

    parser
        .set_language(&language)
        .map_err(|error| format!("Failed to configure parser: {error}"))?;

    let tree = parser
        .parse(&source, None)
        .ok_or_else(|| "Failed to parse source file".to_string())?;

    let mut functions = Vec::new();
    let mut calls = Vec::new();

    collect_nodes(
        tree.root_node(),
        source.as_bytes(),
        &mut functions,
        &mut calls,
    );

    Ok(SourceAnalysis {
        path: path.display().to_string(),
        functions,
        calls,
    })
}

fn language_for_path(path: &Path) -> Result<Language, String> {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("c") => Ok(tree_sitter_c::LANGUAGE.into()),
        Some("cpp") | Some("cc") | Some("cxx") => Ok(tree_sitter_cpp::LANGUAGE.into()),
        _ => Err(format!("Unsupported source file: {}", path.display())),
    }
}

fn collect_nodes(
    node: tree_sitter::Node,
    source: &[u8],
    functions: &mut Vec<String>,
    calls: &mut Vec<String>,
) {
    if node.kind() == "function_definition" {
        if let Some(name) = node
            .child_by_field_name("declarator")
            .and_then(|declarator| find_function_name(declarator, source))
        {
            functions.push(name);
        }
    }

    if node.kind() == "call_expression" {
        if let Some(function) = node.child_by_field_name("function") {
            if let Ok(name) = function.utf8_text(source) {
                if !calls.iter().any(|call| call == name) {
                    calls.push(name.to_string());
                }
            }
        }
    }

    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        collect_nodes(child, source, functions, calls);
    }
}

fn find_function_name(node: tree_sitter::Node, source: &[u8]) -> Option<String> {
    if node.kind() == "identifier" {
        return node.utf8_text(source).ok().map(str::to_owned);
    }

    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if let Some(name) = find_function_name(child, source) {
            return Some(name);
        }
    }

    None
}
