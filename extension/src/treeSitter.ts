import * as path from "path";
import { Language, Parser } from "web-tree-sitter";

export type Expression =
    | {
          type: "integer";
          value: number;
      }
    | {
          type: "string";
          value: string;
      }
    | {
          type: "identifier";
          name: string;
      };

export type Statement =
    | {
          type: "call";
          name: string;
          arguments: Expression[];
      }
    | {
          type: "return";
      };

export interface ParsedFunction {
    name: string;
    body: Statement[];
}

export interface ParsedProgram {
    functions: ParsedFunction[];
}

export async function parseSource(
    extensionPath: string,
    source: string,
    languageName: "c" | "cpp",
): Promise<ParsedProgram> {
    await Parser.init({
        locateFile: (fileName: string) =>
            path.join(
                extensionPath,
                "node_modules",
                "web-tree-sitter",
                fileName,
            ),
    });

    const parser = new Parser();

    try {
        const languagePath = path.join(
            extensionPath,
            "..",
            "parsers",
            languageName === "c" ? "c" : "cpp",
            languageName === "c"
                ? "tree-sitter-c.wasm"
                : "tree-sitter-cpp.wasm",
        );

        const language = await Language.load(languagePath);

        parser.setLanguage(language);

        const tree = parser.parse(source);

        if (!tree) {
            throw new Error(
                "Tree-sitter failed to parse source",
            );
        }

        const program: ParsedProgram = {
            functions: [],
        };

        for (const node of tree.rootNode.namedChildren) {
            if (node.type !== "function_definition") {
                continue;
            }

            const declarator =
                node.childForFieldName("declarator");

            if (!declarator) {
                continue;
            }

            const functionDeclarator =
                declarator.type === "function_declarator"
                    ? declarator
                    : declarator.descendantsOfType(
                        "function_declarator",
                    )[0];

            if (!functionDeclarator) {
                continue;
            }

            const nameNode =
                functionDeclarator.childForFieldName(
                    "declarator",
                );

            if (!nameNode) {
                continue;
            }

            const body =
                node.childForFieldName("body");

            if (!body) {
                continue;
            }

            const parsedFunction: ParsedFunction = {
                name: nameNode.text,
                body: [],
            };

            for (const statement of body.namedChildren) {
                if (
                    statement.type ===
                    "expression_statement"
                ) {
                    const expression =
                        statement.namedChildren[0];

                    if (
                        !expression ||
                        expression.type !==
                            "call_expression"
                    ) {
                        continue;
                    }

                    const functionNode =
                        expression.childForFieldName(
                            "function",
                        );

                    const argumentsNode =
                        expression.childForFieldName(
                            "arguments",
                        );

                    if (
                        !functionNode ||
                        !argumentsNode
                    ) {
                        continue;
                    }

                    const args: Expression[] = [];

                    for (
                        const argument of
                        argumentsNode.namedChildren
                    ) {
                        const parsed =
                            parseExpression(argument);

                        if (parsed) {
                            args.push(parsed);
                        }
                    }

                    parsedFunction.body.push({
                        type: "call",
                        name: functionNode.text,
                        arguments: args,
                    });
                }

                if (
                    statement.type ===
                    "return_statement"
                ) {
                    parsedFunction.body.push({
                        type: "return",
                    });
                }
            }

            program.functions.push(
                parsedFunction,
            );
        }

        tree.delete();

        return program;
    } finally {
        parser.delete();
    }
}

function parseExpression(
    node: any,
): Expression | null {
    if (
        node.type === "number_literal" ||
        node.type === "integer_literal"
    ) {
        const value = Number.parseInt(
            node.text,
            0,
        );

        if (!Number.isNaN(value)) {
            return {
                type: "integer",
                value,
            };
        }
    }

    if (node.type === "string_literal") {
        return {
            type: "string",
            value: node.text.slice(1, -1),
        };
    }

    if (node.type === "identifier") {
        return {
            type: "identifier",
            name: node.text,
        };
    }

    return null;
}