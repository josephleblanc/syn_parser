use crate::parser::types::{GenericParamNode, TypeId, VisibilityKind};
use crate::parser::visitor::utils::attributes::ParsedAttribute;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedAttribute {
    pub name: String,
    pub args: Vec<String>,
    pub value: Option<String>,
}

// Unique ID for a node in the graph
pub type NodeId = usize;

// ANCHOR: ItemFn
// Represents a function definition
#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionNode {
    pub id: NodeId,
    pub name: String,
    pub visibility: VisibilityKind,
    pub parameters: Vec<ParameterNode>,
    pub return_type: Option<TypeId>,
    pub generic_params: Vec<GenericParamNode>,
    pub attributes: Vec<ParsedAttribute>,
    pub docstring: Option<String>,
    pub body: Option<String>,
}
//ANCHOR_END: ItemFn

// Represents a parameter in a function
#[derive(Debug, Serialize, Deserialize)]
pub struct ParameterNode {
    pub id: NodeId,
    pub name: Option<String>,
    pub type_id: TypeId,
    pub is_mutable: bool,
    pub is_self: bool,
}

// Represents a type definition (struct, enum, type alias, or union)
#[derive(Debug, Serialize, Deserialize)]
pub enum TypeDefNode {
    Struct(StructNode),
    Enum(EnumNode),
    TypeAlias(TypeAliasNode),
    Union(UnionNode),
}

// ANCHOR: StructNode
// Represents a struct definition
#[derive(Debug, Serialize, Deserialize)]
pub struct StructNode {
    pub id: NodeId,
    pub name: String,
    pub visibility: VisibilityKind,
    pub fields: Vec<FieldNode>,
    pub generic_params: Vec<GenericParamNode>,
    pub attributes: Vec<ParsedAttribute>, // Replace Vec<String>
    pub docstring: Option<String>,
}
//ANCHOR_END: StructNode

// Represents an enum definition
#[derive(Debug, Serialize, Deserialize)]
pub struct EnumNode {
    pub id: NodeId,
    pub name: String,
    pub visibility: VisibilityKind,
    pub variants: Vec<VariantNode>,
    pub generic_params: Vec<GenericParamNode>,
    pub attributes: Vec<ParsedAttribute>,
    pub docstring: Option<String>,
}

// ANCHOR: field_node
// Represents a field in a struct
#[derive(Debug, Serialize, Deserialize)]
pub struct FieldNode {
    pub id: NodeId,
    pub name: Option<String>,
    pub type_id: TypeId,
    pub visibility: VisibilityKind,
    pub attributes: Vec<ParsedAttribute>,
}
//ANCHOR_END: field_node

// Represents a variant in an enum
#[derive(Debug, Serialize, Deserialize)]
pub struct VariantNode {
    pub id: NodeId,
    pub name: String,
    pub fields: Vec<FieldNode>,
    pub discriminant: Option<String>,
    pub attributes: Vec<ParsedAttribute>,
}

// Represents a type alias (type NewType = OldType)
#[derive(Debug, Serialize, Deserialize)]
pub struct TypeAliasNode {
    pub id: NodeId,
    pub name: String,
    pub visibility: VisibilityKind,
    pub type_id: TypeId,
    pub generic_params: Vec<GenericParamNode>,
    pub attributes: Vec<ParsedAttribute>,
    pub docstring: Option<String>,
}

// Represents a union definition
#[derive(Debug, Serialize, Deserialize)]
pub struct UnionNode {
    pub id: NodeId,
    pub name: String,
    pub visibility: VisibilityKind,
    pub fields: Vec<FieldNode>,
    pub generic_params: Vec<GenericParamNode>,
    pub attributes: Vec<ParsedAttribute>,
    pub docstring: Option<String>,
}

// ANCHOR: ImplNode
// Represents an implementation block
#[derive(Debug, Serialize, Deserialize)]
pub struct ImplNode {
    pub id: NodeId,
    pub self_type: TypeId,
    pub trait_type: Option<TypeId>,
    pub methods: Vec<FunctionNode>,
    pub generic_params: Vec<GenericParamNode>,
}
//ANCHOR_END: ItemImpl

// ANCHOR: TraitNode
// Represents a trait definition
#[derive(Debug, Serialize, Deserialize)]
pub struct TraitNode {
    pub id: NodeId,
    pub name: String,
    pub visibility: VisibilityKind,
    pub methods: Vec<FunctionNode>,
    pub generic_params: Vec<GenericParamNode>,
    pub super_traits: Vec<TypeId>,
    pub attributes: Vec<ParsedAttribute>,
    pub docstring: Option<String>,
}
//ANCHOR_END: TraitNode

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleNode {
    pub id: NodeId,
    pub name: String,
    pub visibility: VisibilityKind,
    pub attributes: Vec<ParsedAttribute>,
    pub docstring: Option<String>,
    pub submodules: Vec<NodeId>,
    pub items: Vec<NodeId>,
    pub imports: Vec<ImportNode>,
    pub exports: Vec<NodeId>,
}

// Represents a constant or static variable
#[derive(Debug, Serialize, Deserialize)]
pub struct ValueNode {
    pub id: NodeId,
    pub name: String,
    pub visibility: VisibilityKind,
    pub type_id: TypeId,
    pub kind: ValueKind,
    pub value: Option<String>,
    pub attributes: Vec<ParsedAttribute>,
    pub docstring: Option<String>,
}

// Represents a macro definition
#[derive(Debug, Serialize, Deserialize)]
pub struct MacroNode {
    pub id: NodeId,
    pub name: String,
    pub visibility: VisibilityKind,
    pub kind: MacroKind,
    pub rules: Vec<MacroRuleNode>,
    pub attributes: Vec<ParsedAttribute>,
    pub docstring: Option<String>,
    pub body: Option<String>,
    pub expansion: Option<String>,       // Track macro expansion
    pub parent_function: Option<NodeId>, // Track containing function
}

// Represents a macro rule
#[derive(Debug, Serialize, Deserialize)]
pub struct MacroRuleNode {
    pub id: NodeId,
    pub pattern: String,
    pub expansion: String,
}

// Different kinds of macros
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum MacroKind {
    DeclarativeMacro,
    ProcedureMacro { kind: ProcMacroKind },
}

// Different kinds of procedural macros
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProcMacroKind {
    Derive,
    ParsedAttribute,
    Function,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValueKind {
    Constant,
    Static { is_mutable: bool },
}

// Represents a module
#[derive(Debug, Serialize, Deserialize)]
pub struct ImportNode {
    pub id: NodeId,
    pub path: Vec<String>,
    pub kind: ImportKind,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ImportKind {
    UseStatement,
    ExternCrate,
}
