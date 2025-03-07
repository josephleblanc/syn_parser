use proc_macro2::TokenStream;
use quote::ToTokens;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File as FsFile;
use std::io::Write;
use std::path::Path;
use syn::ItemMod;
use syn::{
    visit::{self, Visit},
    AngleBracketedGenericArguments, Field, File, FnArg, GenericArgument, Generics, Ident, ItemEnum,
    ItemFn, ItemImpl, ItemStruct, ItemTrait, Lifetime, Pat, PatIdent, PatType, Path as SynPath,
    PathArguments, PathSegment, ReturnType, Type, TypeParam, TypePath, TypeReference, Variant,
    Visibility,
};

// Type ID for internal references
type TypeId = usize;

// Main structure representing the entire code graph
#[derive(Debug, Serialize, Deserialize)]
pub struct CodeGraph {
    // Functions defined in the code
    pub functions: Vec<FunctionNode>,
    // Types (structs, enums) defined in the code
    pub defined_types: Vec<TypeDefNode>,
    // All observed types, including nested and generic types
    pub type_graph: Vec<TypeNode>,
    // Implementation blocks
    pub impls: Vec<ImplNode>,
    // Traits defined in the code
    pub traits: Vec<TraitNode>,
    // Relations between nodes
    pub relations: Vec<Relation>,
}

// Represents a module
#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleNode {
    pub id: NodeId,
    pub name: String,
    pub visibility: VisibilityKind,
    pub attributes: Vec<Attribute>,
    pub docstring: Option<String>,
    pub submodules: Vec<NodeId>,
    pub items: Vec<NodeId>,
}

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
    pub attributes: Vec<Attribute>,
    pub docstring: Option<String>,
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

// Represents a type definition (struct or enum)
#[derive(Debug, Serialize, Deserialize)]
pub enum TypeDefNode {
    Struct(StructNode),
    Enum(EnumNode),
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
    pub attributes: Vec<Attribute>, // Replace Vec<String>
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
    pub attributes: Vec<Attribute>,
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
    pub attributes: Vec<Attribute>,
}
//ANCHOR_END: field_node

// Represents a variant in an enum
#[derive(Debug, Serialize, Deserialize)]
pub struct VariantNode {
    pub id: NodeId,
    pub name: String,
    pub fields: Vec<FieldNode>,
    pub discriminant: Option<String>,
    pub attributes: Vec<Attribute>,
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
    pub attributes: Vec<Attribute>,
    pub docstring: Option<String>,
}
//ANCHOR_END: TraitNode

// ANCHOR: TypeNode
// Represents a type reference with full metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct TypeNode {
    pub id: TypeId,
    pub kind: TypeKind,
    // Reference to related types (e.g., generic arguments)
    pub related_types: Vec<TypeId>,
}
//ANCHOR_END: TypeNode

// Represents a generic parameter
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericParamNode {
    pub id: NodeId,
    pub kind: GenericParamKind,
}

// Represent an attribute
#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub kind: String,          // e.g., "derive", "cfg", "serde"
    pub args: Vec<String>,     // Arguments or parameters of the attribute
    pub value: Option<String>, // Optional value (e.g., for `#[attr = "value"]`)
}

// ANCHOR: generic_param_kind
// Different kinds of generic parameters
#[derive(Debug, Serialize, Deserialize)]
pub enum GenericParamKind {
    Type {
        name: String,
        bounds: Vec<TypeId>,
        default: Option<TypeId>,
    },
    Lifetime {
        name: String,
        bounds: Vec<String>,
    },
    Const {
        name: String,
        type_id: TypeId,
    },
}
//ANCHOR_END: generic_param_kind

// ANCHOR: TypeKind_defn
// Different kinds of types
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TypeKind {
    //ANCHOR_END: TypeKind_defn
    Named {
        path: Vec<String>, // Full path segments
        is_fully_qualified: bool,
    },
    Reference {
        lifetime: Option<String>,
        is_mutable: bool,
        // Type being referenced is in related_types[0]
    },
    Slice {
        // Element type is in related_types[0]
    },
    Array {
        // Element type is in related_types[0]
        size: Option<String>,
    },
    Tuple {
        // Element types are in related_types
    },
    // ANCHOR: ExternCrate
    Function {
        // Parameter types are in related_types (except last one)
        // Return type is in related_types[last]
        is_unsafe: bool,
        is_extern: bool,
        abi: Option<String>,
    },
    //ANCHOR_END: ExternCrate
    Never,
    Inferred,
    RawPointer {
        is_mutable: bool,
        // Pointee type is in related_types[0]
    },
    // ANCHOR: TraitObject
    TraitObject {
        // Trait bounds are in related_types
        dyn_token: bool,
    },
    //ANCHOR_END: TraitObject
    // ANCHOR: ImplTrait
    ImplTrait {
        // Trait bounds are in related_types
    },
    //ANCHOR_END: ImplTrait
    Paren {
        // Inner type is in related_types[0]
    },
    // ANCHOR: ItemMacro
    Macro {
        name: String,
        tokens: String,
    },
    //ANCHOR_END:
    Unknown {
        type_str: String,
    },
}

// Different kinds of visibility
#[derive(Debug, Serialize, Deserialize)]
pub enum VisibilityKind {
    Public,
    Crate,
    Restricted(Vec<String>), // Path components of restricted visibility
    Inherited,               // Default visibility
}

// Represents a relation between nodes
#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    pub source: NodeId,
    pub target: NodeId,
    pub kind: RelationKind,
}

// ANCHOR: Uses
// Different kinds of relations
#[derive(Debug, Serialize, Deserialize)]
pub enum RelationKind {
    FunctionParameter,
    FunctionReturn,
    StructField,
    EnumVariant,
    ImplementsFor,
    ImplementsTrait,
    Inherits,
    References,
    Contains,
    Uses,
}
//ANCHOR_END: Uses

// Unique ID for a node in the graph
pub type NodeId = usize;

// State for the visitor
struct VisitorState {
    code_graph: CodeGraph,
    next_node_id: NodeId,
    next_type_id: TypeId,
    // Maps existing types to their IDs to avoid duplication
    type_map: HashMap<String, TypeId>,
    // add `modules` field AI!
}

impl VisitorState {
    fn new() -> Self {
        Self {
            code_graph: CodeGraph {
                functions: Vec::new(),
                defined_types: Vec::new(),
                type_graph: Vec::new(),
                impls: Vec::new(),
                traits: Vec::new(),
                relations: Vec::new(),
            },
            next_node_id: 0,
            next_type_id: 0,
            type_map: HashMap::new(),
        }
    }

    fn next_node_id(&mut self) -> NodeId {
        let id = self.next_node_id;
        self.next_node_id += 1;
        id
    }

    fn next_type_id(&mut self) -> TypeId {
        let id = self.next_type_id;
        self.next_type_id += 1;
        id
    }

    // Get or create a type ID
    fn get_or_create_type(&mut self, ty: &Type) -> TypeId {
        // Convert type to a string representation for caching
        let type_str = ty.to_token_stream().to_string();

        if let Some(&id) = self.type_map.get(&type_str) {
            return id;
        }

        let (type_kind, related_types) = self.process_type(ty);

        let id = self.next_type_id();
        self.type_map.insert(type_str, id);

        self.code_graph.type_graph.push(TypeNode {
            id,
            kind: type_kind,
            related_types,
        });

        id
    }

    // Process a type and get its kind and related types
    fn process_type(&mut self, ty: &Type) -> (TypeKind, Vec<TypeId>) {
        match ty {
            Type::Path(TypePath { path, qself }) => {
                let mut related_types = Vec::new();
                let segments: Vec<String> = path
                    .segments
                    .iter()
                    .map(|seg| {
                        // Process generic arguments if any
                        if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            args,
                            ..
                        }) = &seg.arguments
                        {
                            for arg in args {
                                match arg {
                                    GenericArgument::Type(arg_type) => {
                                        related_types.push(self.get_or_create_type(arg_type));
                                    }
                                    // Process other generic arguments if needed
                                    _ => {}
                                }
                            }
                        } else if let PathArguments::Parenthesized(parenthesized) = &seg.arguments {
                            // Handle function pointer types like Fn(Args) -> Return
                            for input in &parenthesized.inputs {
                                related_types.push(self.get_or_create_type(input));
                            }
                            if let ReturnType::Type(_, return_ty) = &parenthesized.output {
                                related_types.push(self.get_or_create_type(return_ty));
                            }
                        }

                        seg.ident.to_string()
                    })
                    .collect();

                (
                    TypeKind::Named {
                        path: segments,
                        is_fully_qualified: qself.is_some(),
                    },
                    related_types,
                )
            }
            Type::Reference(TypeReference {
                elem,
                lifetime,
                mutability,
                ..
            }) => {
                let elem_id = self.get_or_create_type(elem);
                (
                    TypeKind::Reference {
                        lifetime: lifetime.as_ref().map(|lt| lt.ident.to_string()),
                        is_mutable: mutability.is_some(),
                    },
                    vec![elem_id],
                )
            }
            Type::Tuple(tuple) => {
                let elem_ids: Vec<TypeId> = tuple
                    .elems
                    .iter()
                    .map(|elem| self.get_or_create_type(elem))
                    .collect();
                (TypeKind::Tuple {}, elem_ids)
            }
            Type::Array(array) => {
                let elem_id = self.get_or_create_type(&array.elem);
                let size = array.len.to_token_stream().to_string();
                (TypeKind::Array { size: Some(size) }, vec![elem_id])
            }
            Type::Slice(slice) => {
                let elem_id = self.get_or_create_type(&slice.elem);
                (TypeKind::Slice {}, vec![elem_id])
            }
            Type::Never(_) => (TypeKind::Never, Vec::new()),
            Type::Infer(_) => (TypeKind::Inferred, Vec::new()),
            Type::Ptr(ptr) => {
                let pointee_id = self.get_or_create_type(&ptr.elem);
                (
                    TypeKind::RawPointer {
                        is_mutable: ptr.mutability.is_some(),
                    },
                    vec![pointee_id],
                )
            }
            Type::BareFn(bare_fn) => {
                let mut related_ids = Vec::new();
                for input in &bare_fn.inputs {
                    related_ids.push(self.get_or_create_type(&input.ty));
                }
                if let ReturnType::Type(_, return_ty) = &bare_fn.output {
                    related_ids.push(self.get_or_create_type(return_ty));
                }
                (
                    TypeKind::Function {
                        is_unsafe: bare_fn.unsafety.is_some(),
                        is_extern: bare_fn.abi.is_some(),
                        abi: bare_fn.abi.as_ref().map(|abi| {
                            abi.name
                                .as_ref()
                                .map_or("C".to_string(), |name| name.value())
                        }),
                    },
                    related_ids,
                )
            }
            Type::Paren(paren) => {
                let inner_id = self.get_or_create_type(&paren.elem);
                (TypeKind::Paren {}, vec![inner_id])
            }
            Type::TraitObject(trait_obj) => {
                let bound_ids: Vec<TypeId> = trait_obj
                    .bounds
                    .iter()
                    .filter_map(|bound| {
                        match bound {
                            syn::TypeParamBound::Trait(trait_bound) => {
                                let path_string = trait_bound.path.to_token_stream().to_string();
                                // Create a synthetic type for the trait bound
                                let bound_id = self.next_type_id();
                                self.code_graph.type_graph.push(TypeNode {
                                    id: bound_id,
                                    kind: TypeKind::Named {
                                        path: trait_bound
                                            .path
                                            .segments
                                            .iter()
                                            .map(|seg| seg.ident.to_string())
                                            .collect(),
                                        is_fully_qualified: false,
                                    },
                                    related_types: Vec::new(),
                                });
                                Some(bound_id)
                            }
                            _ => None,
                        }
                    })
                    .collect();
                (
                    TypeKind::TraitObject {
                        dyn_token: trait_obj.dyn_token.is_some(),
                    },
                    bound_ids,
                )
            }
            Type::ImplTrait(impl_trait) => {
                let bound_ids: Vec<TypeId> = impl_trait
                    .bounds
                    .iter()
                    .filter_map(|bound| {
                        match bound {
                            syn::TypeParamBound::Trait(trait_bound) => {
                                let path_string = trait_bound.path.to_token_stream().to_string();
                                // Create a synthetic type for the trait bound
                                let bound_id = self.next_type_id();
                                self.code_graph.type_graph.push(TypeNode {
                                    id: bound_id,
                                    kind: TypeKind::Named {
                                        path: trait_bound
                                            .path
                                            .segments
                                            .iter()
                                            .map(|seg| seg.ident.to_string())
                                            .collect(),
                                        is_fully_qualified: false,
                                    },
                                    related_types: Vec::new(),
                                });
                                Some(bound_id)
                            }
                            _ => None,
                        }
                    })
                    .collect();
                (TypeKind::ImplTrait {}, bound_ids)
            }
            Type::Macro(type_macro) => (
                TypeKind::Macro {
                    name: type_macro.mac.path.to_token_stream().to_string(),
                    tokens: type_macro.mac.tokens.to_string(),
                },
                Vec::new(),
            ),
            _ => {
                // Handle other types or unknown types
                (
                    TypeKind::Unknown {
                        type_str: ty.to_token_stream().to_string(),
                    },
                    Vec::new(),
                )
            }
        }
    }

    // Convert syn::Visibility to our VisibilityKind
    fn convert_visibility(&self, vis: &Visibility) -> VisibilityKind {
        match vis {
            Visibility::Public(_) => VisibilityKind::Public,
            Visibility::Restricted(restricted) => {
                let path = restricted
                    .path
                    .segments
                    .iter()
                    .map(|seg| seg.ident.to_string())
                    .collect();
                VisibilityKind::Restricted(path)
            }
            Visibility::Inherited => VisibilityKind::Inherited,
        }
    }

    // Process a function parameter
    fn process_fn_arg(&mut self, arg: &FnArg) -> Option<ParameterNode> {
        match arg {
            FnArg::Typed(PatType { pat, ty, .. }) => {
                let type_id = self.get_or_create_type(ty);

                // Extract parameter name and mutability
                let (name, is_mutable) = match &**pat {
                    Pat::Ident(PatIdent {
                        ident, mutability, ..
                    }) => (Some(ident.to_string()), mutability.is_some()),
                    _ => (None, false),
                };

                Some(ParameterNode {
                    id: self.next_node_id(),
                    name,
                    type_id,
                    is_mutable,
                    is_self: false,
                })
            }
            FnArg::Receiver(receiver) => {
                // Create a special self type
                let self_type_id = self.next_type_id();
                let mut related_types = Vec::new();

                // If we have an explicit type for self, include it
                // if let Some(ty_box) = receiver.ty {
                let ty_ref: &syn::Type = &receiver.ty; // Dereference the Box to get &syn::Type
                let inner_type_id = self.get_or_create_type(ty_ref);
                related_types.push(inner_type_id);
                // }

                self.code_graph.type_graph.push(TypeNode {
                    id: self_type_id,
                    kind: TypeKind::Named {
                        path: vec!["Self".to_string()],
                        is_fully_qualified: false,
                    },
                    related_types,
                });

                Some(ParameterNode {
                    id: self.next_node_id(),
                    name: Some("self".to_string()),
                    type_id: self_type_id,
                    is_mutable: receiver.mutability.is_some(),
                    is_self: true,
                })
            }
        }
    }

    // Process generic parameters
    fn process_generics(&mut self, generics: &Generics) -> Vec<GenericParamNode> {
        let mut params = Vec::new();

        for param in &generics.params {
            match param {
                syn::GenericParam::Type(TypeParam {
                    ident,
                    bounds,
                    default,
                    ..
                }) => {
                    let bound_ids: Vec<TypeId> = bounds
                        .iter()
                        .filter_map(|bound| {
                            match bound {
                                syn::TypeParamBound::Trait(trait_bound) => {
                                    // Create a synthetic type for the trait bound
                                    let bound_id = self.next_type_id();
                                    self.code_graph.type_graph.push(TypeNode {
                                        id: bound_id,
                                        kind: TypeKind::Named {
                                            path: trait_bound
                                                .path
                                                .segments
                                                .iter()
                                                .map(|seg| seg.ident.to_string())
                                                .collect(),
                                            is_fully_qualified: false,
                                        },
                                        related_types: Vec::new(),
                                    });
                                    Some(bound_id)
                                }
                                _ => None, // Ignore lifetime bounds for now
                            }
                        })
                        .collect();

                    let default_type = default.as_ref().map(|ty| self.get_or_create_type(ty));

                    params.push(GenericParamNode {
                        id: self.next_node_id(),
                        kind: GenericParamKind::Type {
                            name: ident.to_string(),
                            bounds: bound_ids,
                            default: default_type,
                        },
                    });
                }
                syn::GenericParam::Lifetime(lifetime_def) => {
                    let bounds: Vec<String> = lifetime_def
                        .bounds
                        .iter()
                        .map(|bound| bound.ident.to_string())
                        .collect();

                    params.push(GenericParamNode {
                        id: self.next_node_id(),
                        kind: GenericParamKind::Lifetime {
                            name: lifetime_def.lifetime.ident.to_string(),
                            bounds,
                        },
                    });
                }
                syn::GenericParam::Const(const_param) => {
                    let type_id = self.get_or_create_type(&const_param.ty);

                    params.push(GenericParamNode {
                        id: self.next_node_id(),
                        kind: GenericParamKind::Const {
                            name: const_param.ident.to_string(),
                            type_id,
                        },
                    });
                }
            }
        }

        params
    }

    // Extract doc comments from attributes
    fn extract_docstring(&self, attrs: &[syn::Attribute]) -> Option<String> {
        let doc_lines: Vec<String> = attrs
            .iter()
            .filter(|attr| attr.path().is_ident("doc"))
            .filter_map(|attr| {
                if let Ok(syn::MetaNameValue {
                    value:
                        syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }),
                    ..
                }) = attr.meta.require_name_value()
                {
                    Some(lit_str.value().trim().to_string())
                } else {
                    None
                }
            })
            .collect();

        if doc_lines.is_empty() {
            None
        } else {
            Some(doc_lines.join("\n"))
        }
    }

    // Extract attribute strings
    // fn extract_attributes(&self, attrs: &[syn::Attribute]) -> Vec<String> {
    //     attrs
    //         .iter()
    //         .filter(|attr| !attr.path().is_ident("doc")) // Skip doc comments
    //         .map(|attr| attr.to_token_stream().to_string())
    //         .collect()
    // }

    fn parse_attribute(attr: &syn::Attribute) -> Option<Attribute> {
        let path = attr.path().to_token_stream().to_string();
        let args = match &attr.meta {
            syn::Meta::List(list) => list
                .tokens
                .to_string()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            syn::Meta::NameValue(name_value) => {
                vec![name_value.value.to_token_stream().to_string()]
            }
            _ => Vec::new(),
        };
        Some(Attribute {
            kind: path,
            args,
            value: None,
        })
    }
    fn extract_attributes(&self, attrs: &[syn::Attribute]) -> Vec<Attribute> {
        attrs
            .iter()
            .filter(|attr| !attr.path().is_ident("doc")) // Skip doc comments
            .filter_map(|attr| VisitorState::parse_attribute(attr))
            .collect()
    }
}

// Visitor implementation
struct CodeVisitor<'a> {
    state: &'a mut VisitorState,
}

impl<'a> CodeVisitor<'a> {
    fn new(state: &'a mut VisitorState) -> Self {
        Self { state }
    }
}

impl<'a, 'ast> Visit<'ast> for CodeVisitor<'a> {
    // Visit function definitions
    fn visit_item_fn(&mut self, func: &'ast ItemFn) {
        let fn_id = self.state.next_node_id();
        let fn_name = func.sig.ident.to_string();

        // Process function parameters
        let mut parameters = Vec::new();
        for arg in &func.sig.inputs {
            if let Some(param) = self.state.process_fn_arg(arg) {
                // Add relation between function and parameter
                self.state.code_graph.relations.push(Relation {
                    source: fn_id,
                    target: param.id,
                    kind: RelationKind::FunctionParameter,
                });
                parameters.push(param);
            }
        }

        // Extract return type if it exists
        let return_type = match &func.sig.output {
            ReturnType::Default => None,
            ReturnType::Type(_, ty) => {
                let type_id = self.state.get_or_create_type(ty);
                // Add relation between function and return type
                self.state.code_graph.relations.push(Relation {
                    source: fn_id,
                    target: type_id,
                    kind: RelationKind::FunctionReturn,
                });
                Some(type_id)
            }
        };

        // Process generic parameters
        let generic_params = self.state.process_generics(&func.sig.generics);

        // Extract doc comments and other attributes
        let docstring = self.state.extract_docstring(&func.attrs);
        let attributes = self.state.extract_attributes(&func.attrs);

        // Store function info
        self.state.code_graph.functions.push(FunctionNode {
            id: fn_id,
            name: fn_name,
            visibility: self.state.convert_visibility(&func.vis),
            parameters,
            return_type,
            generic_params,
            attributes,
            docstring,
        });

        // Continue visiting the function body
        visit::visit_item_fn(self, func);
    }

    // Visit struct definitions
    fn visit_item_struct(&mut self, item_struct: &'ast ItemStruct) {
        let struct_id = self.state.next_node_id();
        let struct_name = item_struct.ident.to_string();

        // Process fields
        let mut fields = Vec::new();
        for field in &item_struct.fields {
            let field_id = self.state.next_node_id();
            let field_name = field.ident.as_ref().map(|ident| ident.to_string());
            let type_id = self.state.get_or_create_type(&field.ty);

            let field_node = FieldNode {
                id: field_id,
                name: field_name,
                type_id,
                visibility: self.state.convert_visibility(&field.vis),
                attributes: self.state.extract_attributes(&field.attrs),
            };

            // Add relation between struct and field
            self.state.code_graph.relations.push(Relation {
                source: struct_id,
                target: field_id,
                kind: RelationKind::StructField,
            });

            fields.push(field_node);
        }

        // Process generic parameters
        let generic_params = self.state.process_generics(&item_struct.generics);

        // Extract doc comments and other attributes
        let docstring = self.state.extract_docstring(&item_struct.attrs);
        let attributes = self.state.extract_attributes(&item_struct.attrs);

        // Store struct info
        self.state
            .code_graph
            .defined_types
            .push(TypeDefNode::Struct(StructNode {
                id: struct_id,
                name: struct_name,
                visibility: self.state.convert_visibility(&item_struct.vis),
                fields,
                generic_params,
                attributes,
                docstring,
            }));

        visit::visit_item_struct(self, item_struct);
    }

    // Visit enum definitions
    fn visit_item_enum(&mut self, item_enum: &'ast ItemEnum) {
        let enum_id = self.state.next_node_id();
        let enum_name = item_enum.ident.to_string();

        // Process variants
        let mut variants = Vec::new();
        for variant in &item_enum.variants {
            let variant_id = self.state.next_node_id();
            let variant_name = variant.ident.to_string();

            // Process fields of the variant
            let mut fields = Vec::new();
            match &variant.fields {
                syn::Fields::Named(fields_named) => {
                    for field in &fields_named.named {
                        let field_id = self.state.next_node_id();
                        let field_name = field.ident.as_ref().map(|ident| ident.to_string());
                        let type_id = self.state.get_or_create_type(&field.ty);

                        let field_node = FieldNode {
                            id: field_id,
                            name: field_name,
                            type_id,
                            visibility: self.state.convert_visibility(&field.vis),
                            attributes: self.state.extract_attributes(&field.attrs),
                        };

                        fields.push(field_node);
                    }
                }
                syn::Fields::Unnamed(fields_unnamed) => {
                    for (i, field) in fields_unnamed.unnamed.iter().enumerate() {
                        let field_id = self.state.next_node_id();
                        let type_id = self.state.get_or_create_type(&field.ty);

                        let field_node = FieldNode {
                            id: field_id,
                            name: None, // Tuple fields don't have names
                            type_id,
                            visibility: self.state.convert_visibility(&field.vis),
                            attributes: self.state.extract_attributes(&field.attrs),
                        };

                        fields.push(field_node);
                    }
                }
                syn::Fields::Unit => {
                    // Unit variants don't have fields
                }
            }

            // Extract discriminant if any
            let discriminant = variant
                .discriminant
                .as_ref()
                .map(|(_, expr)| expr.to_token_stream().to_string());

            let variant_node = VariantNode {
                id: variant_id,
                name: variant_name,
                fields,
                discriminant,
                attributes: self.state.extract_attributes(&variant.attrs),
            };

            // Add relation between enum and variant
            self.state.code_graph.relations.push(Relation {
                source: enum_id,
                target: variant_id,
                kind: RelationKind::EnumVariant,
            });

            variants.push(variant_node);
        }

        // Process generic parameters
        let generic_params = self.state.process_generics(&item_enum.generics);

        // Extract doc comments and other attributes
        let docstring = self.state.extract_docstring(&item_enum.attrs);
        let attributes = self.state.extract_attributes(&item_enum.attrs);

        // Store enum info
        self.state
            .code_graph
            .defined_types
            .push(TypeDefNode::Enum(EnumNode {
                id: enum_id,
                name: enum_name,
                visibility: self.state.convert_visibility(&item_enum.vis),
                variants,
                generic_params,
                attributes,
                docstring,
            }));

        visit::visit_item_enum(self, item_enum);
    }

    // Visit impl blocks
    fn visit_item_impl(&mut self, item_impl: &'ast ItemImpl) {
        let impl_id = self.state.next_node_id();

        // Process self type
        let self_type_id = self.state.get_or_create_type(&item_impl.self_ty);

        // Process trait type if it's a trait impl
        let trait_type_id = item_impl.trait_.as_ref().map(|(_, path, _)| {
            let ty = Type::Path(TypePath {
                qself: None,
                path: path.clone(),
            });
            self.state.get_or_create_type(&ty)
        });

        // Process methods
        let mut methods = Vec::new();
        for item in &item_impl.items {
            if let syn::ImplItem::Fn(method) = item {
                let method_node_id = self.state.next_node_id();
                let method_name = method.sig.ident.to_string();

                // Process method parameters
                let mut parameters = Vec::new();
                for arg in &method.sig.inputs {
                    if let Some(param) = self.state.process_fn_arg(arg) {
                        // Add relation between method and parameter
                        self.state.code_graph.relations.push(Relation {
                            source: method_node_id,
                            target: param.id,
                            kind: RelationKind::FunctionParameter,
                        });
                        parameters.push(param);
                    }
                }

                // Extract return type if it exists
                let return_type = match &method.sig.output {
                    ReturnType::Default => None,
                    ReturnType::Type(_, ty) => {
                        let type_id = self.state.get_or_create_type(ty);
                        // Add relation between method and return type
                        self.state.code_graph.relations.push(Relation {
                            source: method_node_id,
                            target: type_id,
                            kind: RelationKind::FunctionReturn,
                        });
                        Some(type_id)
                    }
                };

                // Process generic parameters for methods
                let generic_params = self.state.process_generics(&method.sig.generics);

                // Extract doc comments and other attributes for methods
                let docstring = self.state.extract_docstring(&method.attrs);
                let attributes = self.state.extract_attributes(&method.attrs);

                // Store method info
                let method_node = FunctionNode {
                    id: method_node_id,
                    name: method_name,
                    visibility: self.state.convert_visibility(&method.vis),
                    parameters,
                    return_type,
                    generic_params,
                    attributes,
                    docstring,
                };
                methods.push(method_node);
            }
        }

        // Process generic parameters for impl block
        let generic_params = self.state.process_generics(&item_impl.generics);

        // Store impl info
        let impl_node = ImplNode {
            id: impl_id,
            self_type: self_type_id,
            trait_type: trait_type_id,
            methods,
            generic_params,
        };
        self.state.code_graph.impls.push(impl_node);

        // Add relation: ImplementsFor or ImplementsTrait
        let relation_kind = if trait_type_id.is_some() {
            RelationKind::ImplementsTrait
        } else {
            RelationKind::ImplementsFor
        };
        self.state.code_graph.relations.push(Relation {
            source: impl_id,
            target: self_type_id,
            kind: relation_kind,
        });
        if let Some(trait_type_id) = trait_type_id {
            self.state.code_graph.relations.push(Relation {
                source: impl_id,
                target: trait_type_id,
                kind: RelationKind::ImplementsTrait,
            });
        }

        visit::visit_item_impl(self, item_impl);
    }

    // Visit trait definitions
    fn visit_item_trait(&mut self, item_trait: &'ast ItemTrait) {
        let trait_id = self.state.next_node_id();
        let trait_name = item_trait.ident.to_string();

        // Process methods
        let mut methods = Vec::new();
        for item in &item_trait.items {
            if let syn::TraitItem::Fn(method) = item {
                let method_node_id = self.state.next_node_id();
                let method_name = method.sig.ident.to_string();

                // Process method parameters
                let mut parameters = Vec::new();
                for arg in &method.sig.inputs {
                    if let Some(param) = self.state.process_fn_arg(arg) {
                        // Add relation between method and parameter
                        self.state.code_graph.relations.push(Relation {
                            source: method_node_id,
                            target: param.id,
                            kind: RelationKind::FunctionParameter,
                        });
                        parameters.push(param);
                    }
                }

                // Extract return type if it exists
                let return_type = match &method.sig.output {
                    ReturnType::Default => None,
                    ReturnType::Type(_, ty) => {
                        let type_id = self.state.get_or_create_type(ty);
                        // Add relation between method and return type
                        self.state.code_graph.relations.push(Relation {
                            source: method_node_id,
                            target: type_id,
                            kind: RelationKind::FunctionReturn,
                        });
                        Some(type_id)
                    }
                };

                // Process generic parameters for methods
                let generic_params = self.state.process_generics(&method.sig.generics);

                // Extract doc comments and other attributes for methods
                let docstring = self.state.extract_docstring(&method.attrs);
                let attributes = self.state.extract_attributes(&method.attrs);

                // Store method info
                let method_node = FunctionNode {
                    id: method_node_id,
                    name: method_name,
                    visibility: VisibilityKind::Public, // Trait methods are always public
                    parameters,
                    return_type,
                    generic_params,
                    attributes,
                    docstring,
                };
                methods.push(method_node);
            }
        }

        // Process generic parameters
        let generic_params = self.state.process_generics(&item_trait.generics);

        // Process super traits
        let super_traits: Vec<TypeId> = item_trait
            .supertraits
            .iter()
            .map(|bound| {
                let ty = Type::TraitObject(syn::TypeTraitObject {
                    dyn_token: None,
                    bounds: syn::punctuated::Punctuated::from_iter(vec![bound.clone()]),
                });
                self.state.get_or_create_type(&ty)
            })
            .collect();

        // Extract doc comments and other attributes
        let docstring = self.state.extract_docstring(&item_trait.attrs);
        let attributes = self.state.extract_attributes(&item_trait.attrs);

        // Store trait info
        let trait_node = TraitNode {
            id: trait_id,
            name: trait_name,
            visibility: self.state.convert_visibility(&item_trait.vis),
            methods,
            generic_params,
            super_traits: super_traits.clone(),
            attributes,
            docstring,
        };
        self.state.code_graph.traits.push(trait_node);

        // Add relation for super traits
        for super_trait_id in &super_traits {
            self.state.code_graph.relations.push(Relation {
                source: trait_id,
                target: *super_trait_id,
                kind: RelationKind::Inherits,
            });
        }

        visit::visit_item_trait(self, item_trait);
    }

    fn visit_item_mod(&mut self, module: &'ast ItemMod) {
        // Extract module information
        let module_id = self.state.next_node_id();
        let module_name = module.ident.to_string();

        // Process inner items if available
        let mut submodules = Vec::new();
        let mut items = Vec::new();

        if let Some((_, mod_items)) = &module.content {
            for item in mod_items {
                let item_id = self.state.next_node_id();
                items.push(item_id);

                match item {
                    syn::Item::Fn(func) => {
                        self.visit_item_fn(func);
                    }
                    syn::Item::Struct(strct) => {
                        self.visit_item_struct(strct);
                    }
                    syn::Item::Enum(enm) => {
                        self.visit_item_enum(enm);
                    }
                    syn::Item::Impl(impl_block) => {
                        self.visit_item_impl(impl_block);
                    }
                    syn::Item::Trait(trt) => {
                        self.visit_item_trait(trt);
                    }
                    syn::Item::Mod(md) => {
                        submodules.push(item_id); // Add to submodules
                        self.visit_item_mod(md); // Recursive call
                    }
                    // Add other item types as needed
                    _ => {}
                }
            }
        }

        // Add module to graph
        self.state.code_graph.modules.push(ModuleNode {
            id: module_id,
            name: module_name,
            visibility: self.state.convert_visibility(&module.vis),
            attributes: self.state.extract_attributes(&module.attrs),
            docstring: self.state.extract_docstring(&module.attrs),
            submodules,
            items,
        });

        // Continue visiting inner items (this is redundant now, remove it)
        // visit::visit_item_mod(self, module);
    }
}

pub fn analyze_code(file_path: &Path) -> Result<CodeGraph, syn::Error> {
    let file = syn::parse_file(&std::fs::read_to_string(file_path).unwrap())?;
    let mut visitor_state = VisitorState::new();
    let mut visitor = CodeVisitor::new(&mut visitor_state);
    visitor.visit_file(&file);
    Ok(visitor_state.code_graph)
}

pub fn save_graph(code_graph: &CodeGraph, output_path: &Path) -> std::io::Result<()> {
    let pretty_config = PrettyConfig::default();
    let ron_string = to_string_pretty(code_graph, pretty_config).expect("Serialization failed");

    let mut output_file = FsFile::create(output_path)?;
    output_file.write_all(ron_string.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_analyzer() {
        let input_path = PathBuf::from("test_data/sample.rs");
        let output_path = PathBuf::from("code_graph.ron");

        let code_graph_result = analyze_code(&input_path);
        assert!(code_graph_result.is_ok());

        let code_graph = code_graph_result.unwrap();
        save_graph(&code_graph, &output_path).expect("Failed to save graph");

        // Here you can add assertions to check the structure of the generated graph
        // For example, check the number of functions, structs, traits, etc.
        assert!(!code_graph.functions.is_empty());
        assert!(!code_graph.defined_types.is_empty());
        assert!(!code_graph.traits.is_empty());
        assert!(!code_graph.impls.is_empty());

        println!("Code graph saved to {:?}", output_path);
    }
}
