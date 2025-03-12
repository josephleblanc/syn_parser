# Rust Code Graph Parser

## Project Goal

The Rust Code Graph Parser is a tool designed to analyze Rust source code and generate a structured representation of the code's components and their relationships. This representation, called a "Code Graph," captures the structure and semantics of Rust code in a way that's easy to query and navigate programmatically. The intention is to use this project as part of an RAG for an LLM that does code generation and refactoring of potentially large rust code repositories.

## Key Features

1. **Comprehensive Parsing**: Parse Rust code constructs including structs, enums, traits, functions, impl blocks, and modules.
2. **Relationship Tracking**: Capture relationships between code elements (e.g., trait implementations, type references).
3. **Type System Representation**: Model Rust's rich type system, including generics, lifetimes, and trait bounds.
4. **Serialization**: Export the code graph to formats like RON for persistence or further analysis.
5. **Modular Testing**: Ensure correctness through focused, modular tests for each aspect of the parser.

## Use Cases

- Static code analysis tools
- Documentation generators
- Code visualization tools
- Refactoring assistants
- Code navigation and search tools
- Educational tools for learning Rust

This project aims to provide a solid foundation for tools that need to understand and manipulate Rust code at a semantic level, beyond what's possible with simple text-based approaches.
