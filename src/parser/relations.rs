use std::fmt;

use crate::parser::graph_ids::GraphNodeId;
use crate::parser::nodes::{NodeId, TraitId};
use crate::parser::types::TypeId;
use serde::{Deserialize, Serialize};

#[cfg(feature = "concurrency_migration")]
#[derive(Debug, Serialize, Deserialize)]
pub struct RelationBatch {
    pub version: u32, // Schema version for evolution
    pub relations: Vec<Relation>,
    pub estimated_size: usize,
    pub source_hash: [u8; 32], // Blake3 hash of source code
}
#[cfg(feature = "concurrency_migration")]
impl RelationBatch {
    /// Serialize relations to JSON for external storage
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.relations).expect("JSON serialization failed")
    }

    /// Convert to Cozo-compatible tuples (for future integration)
    pub fn to_cozo_tuples(&self) -> Vec<serde_json::Value> {
        self.relations
            .iter()
            .map(|rel| {
                cozo::Array::from(vec![
                    rel.source.into(),
                    rel.target.into(),
                    serde_json::json!({
                        "source": rel.source,
                        "target": rel.target,
                        "kind": rel.kind.to_string(),
                        "uuid": self.content_uuid().to_string(),
                        "embedding": blake3::hash(rel.kind.to_string().as_bytes()).as_bytes()
                    }),
                ])
            })
            .collect()
    }

    /// Batch format for CozoDB ingestion
    pub fn to_cozo_batch(&self) -> cozo::DataFrame {
        cozo::DataFrame::from_rows(
            self.relations
                .iter()
                .map(|r| {
                    cozo::Row::from(vec![
                        r.source.into(),
                        r.target.into(),
                        r.kind.to_string(),
                        self.content_uuid().to_string(),
                    ])
                })
                .collect(),
        )
    }

    /// Content-based UUID for batch tracking                                     
    pub fn content_uuid(&self) -> uuid::Uuid {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.source_hash);
        hasher.update(&self.estimated_size.to_le_bytes());
        let hash = hasher.finalize();

        uuid::Uuid::from_slice(hash.as_bytes()).unwrap()
    }
}

// ANCHOR: Relation
// Represents a relation between nodes
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Relation {
    pub graph_source: GraphNodeId,
    pub graph_target: GraphNodeId,
    pub kind: RelationKind,
}
//ANCHOR_END: Relation

#[derive(Debug, thiserror::Error)]
pub enum RelationError {
    #[error("Invalid source type for {kind}. Expected {expected}, found {found}")]
    InvalidSourceType {
        kind: RelationKind,
        expected: &'static str,
        found: &'static str,
    },

    #[error("Invalid target type for {kind}. Expected {expected}, found {found}")]
    InvalidTargetType {
        kind: RelationKind,
        expected: &'static str,
        found: &'static str,
    },

    #[error("Missing required trait ID for {kind:?} relation")]
    MissingTraitId { kind: RelationKind },

    #[error("Invalid node reference {node_id:?} in {kind:?} relation")]
    InvalidNodeReference { kind: RelationKind, node_id: NodeId },

    #[error("Missing required ID for {kind:?} relation")]
    MissingId {
        kind: RelationKind,
        id_type: &'static str,
    },

    #[error(
        "Circular dependency detected between {source_id:?} and {target_id:?} 
 in {kind:?} relation"
    )]
    CircularDependency {
        kind: RelationKind,
        source_id: GraphNodeId,
        target_id: GraphNodeId,
    },

    #[error("Generic constraint violation in {kind:?} relation: {message}")]
    GenericConstraintViolation { kind: RelationKind, message: String },

    #[error("Invalid implementation relationship between {source} and {target}")]
    InvalidImplementation {
        source: GraphNodeId,
        target: GraphNodeId,
        kind: RelationKind,
    },
}
impl Relation {
    pub fn new(
        source: impl Into<GraphNodeId> + Clone + Copy,
        target: impl Into<GraphNodeId> + Clone + Copy,
        kind: RelationKind,
    ) -> Self {
        let source_val = source.into();
        let target_val = target.into();
        Self {
            graph_source: source_val,
            graph_target: target_val,
            kind,
        }
    }
    pub fn validate(&self) -> Result<(), RelationError> {
        match self.kind {
            RelationKind::ImplementsTrait(trait_id) => {
                if trait_id.unique_id == 0 {
                    return Err(RelationError::MissingTraitId {
                        kind: self.kind.clone(),
                    });
                }
                self.validate_types(
                    "Type",
                    "Trait",
                    RelationVariant::Type,
                    RelationVariant::Trait,
                )?
            }
            RelationKind::FunctionParameter => self.validate_types(
                "Function",
                "Type",
                RelationVariant::Node,
                RelationVariant::Type,
            )?,
            RelationKind::FunctionReturn => self.validate_types(
                "Function",
                "Type",
                RelationVariant::Node,
                RelationVariant::Type,
            )?,
            RelationKind::StructField => self.validate_types(
                "Struct",
                "Type",
                RelationVariant::Node,
                RelationVariant::Type,
            )?,
            RelationKind::EnumVariant => {
                self.validate_types("Enum", "Type", RelationVariant::Node, RelationVariant::Type)?
            }
            RelationKind::ImplementsFor => self.validate_types(
                "Trait",
                "Type",
                RelationVariant::Trait,
                RelationVariant::Type,
            )?,
            RelationKind::Inherits => {
                self.validate_types("Type", "Type", RelationVariant::Type, RelationVariant::Type)?;
                self.check_circular_dependency()?
            }
            RelationKind::References => {
                self.validate_types("Node", "Node", RelationVariant::Node, RelationVariant::Node)?
            }
            RelationKind::Contains => {
                self.validate_types(
                    "Container",
                    "Contained",
                    RelationVariant::Node,
                    RelationVariant::Node,
                )?;
                self.check_circular_dependency()?
            }
            RelationKind::TypeDefinition => {
                self.validate_types("Node", "Type", RelationVariant::Node, RelationVariant::Type)?
            }
            RelationKind::Uses => {
                self.validate_types("Node", "Type", RelationVariant::Node, RelationVariant::Type)?
            }
            RelationKind::ValueType => self.validate_types(
                "Node",
                "Primitive",
                RelationVariant::Node,
                RelationVariant::Type,
            )?,
            RelationKind::MacroUse => self.validate_types(
                "Invocation",
                "Definition",
                RelationVariant::Node,
                RelationVariant::Node,
            )?,
            RelationKind::MacroExpansion => self.validate_types(
                "Macro",
                "Expanded",
                RelationVariant::Node,
                RelationVariant::Node,
            )?,
            RelationKind::MacroDefinition => self.validate_types(
                "Macro",
                "Signature",
                RelationVariant::Node,
                RelationVariant::Type,
            )?,
            RelationKind::MacroInvocation => self.validate_types(
                "Caller",
                "Macro",
                RelationVariant::Node,
                RelationVariant::Node,
            )?,
            RelationKind::GenericParameter => self.validate_types(
                "Generic",
                "Type",
                RelationVariant::Node,
                RelationVariant::Type,
            )?,
            RelationKind::Returns => self.validate_types(
                "Function",
                "Type",
                RelationVariant::Node,
                RelationVariant::Type,
            )?,
            RelationKind::HasType => {
                self.validate_types("Node", "Type", RelationVariant::Node, RelationVariant::Type)?
            }
        }
        Ok(())
    }

    fn validate_types(
        &self,
        expected_source: &'static str,
        expected_target: &'static str,
        expected_source_variant: RelationVariant,
        expected_target_variant: RelationVariant,
    ) -> Result<(), RelationError> {
        let actual_source = RelationVariant::from(&self.graph_source);
        let actual_target = RelationVariant::from(&self.graph_target);

        if actual_source != expected_source_variant {
            return Err(RelationError::InvalidSourceType {
                kind: self.kind.clone(),
                expected: expected_source,
                found: self.source.type_name(),
            });
        }

        if actual_target != expected_target_variant {
            return Err(RelationError::InvalidTargetType {
                kind: self.kind.clone(),
                expected: expected_target,
                found: self.source.type_name(),
            });
        }

        Ok(())
    }

    fn check_circular_dependency(&self) -> Result<(), RelationError> {
        if self.source.id() == self.target.id() {
            Err(RelationError::CircularDependency {
                kind: self.kind.clone(),
                source_id: self.source.into(),
                target_id: self.target.into(),
            })
        } else {
            Ok(())
        }
    }
}
/// Represents valid sources for graph relationships
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum RelationSource {
    /// Relationship originates from a general node
    Node(NodeId),
    /// Relationship originates from a trait definition
    Trait(TraitId),
    /// Relationship originates from a type definition
    Type(TypeId),
}
// Add this trait definition
pub trait Identifier {
    fn id(&self) -> usize;
}

// Implement for RelationSource
impl Identifier for RelationSource {
    fn id(&self) -> usize {
        match self {
            RelationSource::Node(id) => id.0,
            RelationSource::Trait(id) => id.0,
            RelationSource::Type(id) => id.0,
        }
    }
}

// Implement for RelationTarget
impl Identifier for RelationTarget {
    fn id(&self) -> usize {
        match self {
            RelationTarget::Node(id) => id.0,
            RelationTarget::Type(id) => id.0,
            RelationTarget::Trait(id) => id.0,
        }
    }
}

#[derive(PartialEq)]
pub enum RelationVariant {
    Node,
    Trait,
    Type,
}

/// Helper function for matching arms of RelationSource enum
// Implement conversion from RelationSource
impl From<&RelationSource> for RelationVariant {
    fn from(source: &RelationSource) -> Self {
        match source {
            RelationSource::Node(_) => RelationVariant::Node,
            RelationSource::Trait(_) => RelationVariant::Trait,
            RelationSource::Type(_) => RelationVariant::Type,
        }
    }
}
/// Helper function for matching arms of RelationTarget enum
// Implement conversion from RelationTarget
impl From<&RelationTarget> for RelationVariant {
    fn from(source: &RelationTarget) -> Self {
        match source {
            RelationTarget::Node(_) => RelationVariant::Node,
            RelationTarget::Trait(_) => RelationVariant::Trait,
            RelationTarget::Type(_) => RelationVariant::Type,
        }
    }
}

impl From<NodeId> for RelationSource {
    fn from(id: NodeId) -> Self {
        RelationSource::Node(id)
    }
}

impl From<TraitId> for RelationSource {
    fn from(id: TraitId) -> Self {
        RelationSource::Trait(id)
    }
}
impl RelationSource {
    pub fn type_name(&self) -> &'static str {
        match self {
            RelationSource::Node(_) => "Node",
            RelationSource::Trait(_) => "Trait",
            RelationSource::Type(_) => "Type",
        }
    }
}

impl From<RelationSource> for GraphNodeId {
    fn from(source: RelationSource) -> Self {
        match source {
            RelationSource::Node(id) => GraphNodeId::from(id),
            RelationSource::Trait(id) => GraphNodeId::from(id),
            RelationSource::Type(type_id) => GraphNodeId::from(type_id),
        }
    }
}

impl From<RelationTarget> for GraphNodeId {
    fn from(target: RelationTarget) -> Self {
        match target {
            RelationTarget::Node(id) => GraphNodeId::from(id),
            RelationTarget::Type(id) => GraphNodeId::from(id),
            RelationTarget::Trait(id) => GraphNodeId::from(id),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum RelationTarget {
    Node(NodeId),
    Type(TypeId),
    Trait(TraitId),
}

impl From<TypeId> for RelationTarget {
    fn from(id: TypeId) -> Self {
        RelationTarget::Type(id)
    }
}

impl From<TraitId> for RelationTarget {
    fn from(id: TraitId) -> Self {
        RelationTarget::Trait(id)
    }
}

impl RelationTarget {
    pub fn type_name(&self) -> &'static str {
        match self {
            RelationTarget::Node(_) => "Node",
            RelationTarget::Type(_) => "Type",
            RelationTarget::Trait(_) => "Trait",
        }
    }
}

// ANCHOR: Uses
// Different kinds of relations
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum RelationKind {
    FunctionParameter,
    FunctionReturn,
    StructField,
    EnumVariant,
    ImplementsFor,
    ImplementsTrait(GraphNodeId), // TraitId type parameter added
    Inherits,
    References,
    Contains,
    TypeDefinition,
    Uses,
    ValueType,
    MacroUse,
    MacroExpansion,
    MacroDefinition,
    MacroInvocation,
    GenericParameter,
    Returns,
    HasType,
}
//ANCHOR_END: Uses

impl fmt::Display for RelationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelationKind::FunctionParameter => write!(f, "FunctionParameter"),
            RelationKind::FunctionReturn => write!(f, "FunctionReturn"),
            RelationKind::StructField => write!(f, "StructField"),
            RelationKind::EnumVariant => write!(f, "EnumVariant"),
            RelationKind::ImplementsFor => write!(f, "ImplementsFor"),
            RelationKind::ImplementsTrait(t) => write!(f, "ImplementsTrait({:?})", t),
            RelationKind::Inherits => write!(f, "Inherits"),
            RelationKind::References => write!(f, "References"),
            RelationKind::Contains => write!(f, "Contains"),
            RelationKind::TypeDefinition => write!(f, "TypeDefinition"),
            RelationKind::Uses => write!(f, "Uses"),
            RelationKind::ValueType => write!(f, "ValueType"),
            RelationKind::MacroUse => write!(f, "MacroUse"),
            RelationKind::MacroExpansion => write!(f, "MacroExpansion"),
            RelationKind::MacroDefinition => write!(f, "MacroDefinition"),
            RelationKind::MacroInvocation => write!(f, "MacroInvocation"),
            RelationKind::GenericParameter => write!(f, "GenericParameter"),
            RelationKind::Returns => write!(f, "Returns"),
            RelationKind::HasType => write!(f, "HasType"),
        }
    }
}
