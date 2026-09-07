use std::path::Path;

use tree_sitter::{Language, Node, Parser};

use crate::ir::{Expression, Function, Program, Statement};

pub fn parse_file(path: &Path) -> Result<Program, String> {
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

    collect_functions(tree.root_node(), source.as_bytes(), &mut functions)?;

    Ok(Program { functions })
}

fn language_for_path(path: &Path) -> Result<Language, String> {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("c") => Ok(tree_sitter_c::LANGUAGE.into()),
        Some("cpp") | Some("cc") | Some("cxx") => Ok(tree_sitter_cpp::LANGUAGE.into()),
        _ => Err(format!("Unsupported source file: {}", path.display())),
    }
}

fn collect_functions(
    node: Node,
    source: &[u8],
    functions: &mut Vec<Function>,
) -> Result<(), String> {
    if node.kind() == "function_definition" {
        let declarator = node
            .child_by_field_name("declarator")
            .ok_or_else(|| "Function declarator not found".to_string())?;

        let name = find_function_name(declarator, source)
            .ok_or_else(|| "Function name not found".to_string())?;

        let body = node
            .child_by_field_name("body")
            .ok_or_else(|| "Function body not found".to_string())?;

        functions.push(Function {
            name,
            body: parse_statements(body, source)?,
        });

        return Ok(());
    }

    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        collect_functions(child, source, functions)?;
    }

    Ok(())
}

fn parse_statements(node: Node, source: &[u8]) -> Result<Vec<Statement>, String> {
    let mut statements = Vec::new();
    let mut cursor = node.walk();

    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "expression_statement" => {
                let Some(call) = child.named_child(0) else {
                    continue;
                };

                if call.kind() != "call_expression" {
                    continue;
                }

                let Some(function) = call.child_by_field_name("function") else {
                    continue;
                };

                let name = function
                    .utf8_text(source)
                    .map_err(|error| error.to_string())?
                    .to_string();

                let mut arguments = Vec::new();

                if let Some(arguments_node) = call.child_by_field_name("arguments") {
                    let mut arguments_cursor = arguments_node.walk();

                    for argument in arguments_node.named_children(&mut arguments_cursor) {
                        if argument.kind() == "string_literal" {
                            let text = argument
                                .utf8_text(source)
                                .map_err(|error| error.to_string())?;

                            let text = text
                                .strip_prefix('"')
                                .and_then(|text| text.strip_suffix('"'))
                                .unwrap_or(text);

                            let text = text
                                .replace("\\n", "\n")
                                .replace("\\t", "\t")
                                .replace("\\r", "\r")
                                .replace("\\\"", "\"")
                                .replace("\\\\", "\\");

                            arguments.push(Expression::String(text));
                        }

                    if argument.kind() == "identifier" {
                            let text = argument
                                .utf8_text(source)
                                .map_err(|error| error.to_string())?;

                            arguments.push(Expression::Identifier(text.to_string()));
                        }
                        
                    if argument.kind() == "number_literal" {
                        let text = argument
                            .utf8_text(source)
                            .map_err(|error| error.to_string())?;

                        let value = text
                            .parse::<i64>()
                            .map_err(|error| error.to_string())?;

                        arguments.push(Expression::Integer(value));
                        }
                    }
                }

                statements.push(Statement::Call { name, arguments });
            }

            "return_statement" => {
                statements.push(Statement::Return);
            }

            _ => {}
        }
    }

    Ok(statements)
}

fn find_function_name(node: Node, source: &[u8]) -> Option<String> {
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
