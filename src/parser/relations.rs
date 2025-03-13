use std::fmt;

use crate::parser::graph_ids::GraphNodeId;
use crate::parser::nodes::{NodeId, TraitId};
use crate::parser::types::TypeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RelationBatch {
    pub version: u32, // Schema version for evolution
    pub relations: Vec<Relation>,
    pub estimated_size: usize,
    pub source_hash: [u8; 32], // Blake3 hash of source code
}

impl RelationBatch {
    /// Convert to IndraDB edges with UUID endpoints                              
    pub fn to_indradb_edges(&self) -> Vec<indradb::Edge> {
        self.relations
            .iter()
            .map(|rel| {
                indradb::Edge::new(rel.source.into(), rel.target.into(), rel.kind.to_string())
            })
            .collect()
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
    pub source: RelationSource,
    pub target: RelationTarget,
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
        source: impl Into<RelationSource> + Clone + Copy,
        target: impl Into<RelationTarget> + Clone + Copy,
        kind: RelationKind,
    ) -> Self {
        let source_val = source.into();
        let target_val = target.into();
        Self {
            source: source_val,
            target: target_val,
            kind,
        }
    }
    pub fn validate(&self) -> Result<(), RelationError> {
        match self.kind {
            RelationKind::ImplementsTrait(_) => {
                if !matches!(
                    (self.source, self.target),
                    (RelationSource::Type(_), RelationTarget::Trait(_))
                ) {
                    return Err(RelationError::InvalidImplementation);
                }
            }
            RelationKind::ImplementsTrait(trait_id) => {
                // Validate trait ID exists
                if trait_id.0 == 0 {
                    return Err(RelationError::MissingTraitId {
                        kind: self.kind.clone(),
                    });
                }

                // Validate type -> trait relationship
                self.validate_types(
                    "Type",
                    "Trait",
                    RelationSource::Type(_),
                    RelationTarget::Trait(_),
                )?
            }
            RelationKind::FunctionParameter => self.validate_types(
                "Function",
                "Type",
                RelationSource::Node(_),
                RelationTarget::Type(_),
            )?,
            RelationKind::FunctionReturn => self.validate_types(
                "Function",
                "Type",
                RelationSource::Node(_),
                RelationTarget::Type(_),
            )?,
            RelationKind::StructField => self.validate_types(
                "Struct",
                "Type",
                RelationSource::Node(_),
                RelationTarget::Type(_),
            )?,
            RelationKind::EnumVariant => self.validate_types(
                "Enum",
                "Type",
                RelationSource::Node(_),
                RelationTarget::Type(_),
            )?,
            RelationKind::ImplementsFor => self.validate_types(
                "Trait",
                "Type",
                RelationSource::Trait(_),
                RelationTarget::Type(_),
            )?,
            RelationKind::Inherits => {
                self.validate_types(
                    "Type",
                    "Type",
                    RelationSource::Type(_),
                    RelationTarget::Type(_),
                )?;
                self.check_circular_dependency()?
            }
            RelationKind::References => self.validate_types(
                "Node",
                "Node",
                RelationSource::Node(_),
                RelationTarget::Node(_),
            )?,
            RelationKind::Contains => {
                self.validate_types(
                    "Container",
                    "Contained",
                    RelationSource::Node(_),
                    RelationTarget::Node(_),
                )?;
                self.check_circular_dependency()?
            }
            RelationKind::TypeDefinition => self.validate_types(
                "Node",
                "Type",
                RelationSource::Node(_),
                RelationTarget::Type(_),
            )?,
            RelationKind::Uses => self.validate_types(
                "Node",
                "Type",
                RelationSource::Node(_),
                RelationTarget::Type(_),
            )?,
            RelationKind::ValueType => self.validate_types(
                "Node",
                "Primitive",
                RelationSource::Node(_),
                RelationTarget::Type(_),
            )?,
            RelationKind::MacroUse => self.validate_types(
                "Invocation",
                "Definition",
                RelationSource::Node(_),
                RelationTarget::Node(_),
            )?,
            RelationKind::MacroExpansion => self.validate_types(
                "Macro",
                "Expanded",
                RelationSource::Node(_),
                RelationTarget::Node(_),
            )?,
            RelationKind::MacroDefinition => self.validate_types(
                "Macro",
                "Signature",
                RelationSource::Node(_),
                RelationTarget::Type(_),
            )?,
            RelationKind::MacroInvocation => self.validate_types(
                "Caller",
                "Macro",
                RelationSource::Node(_),
                RelationTarget::Node(_),
            )?,
            RelationKind::GenericParameter => self.validate_types(
                "Generic",
                "Type",
                RelationSource::Node(_),
                RelationTarget::Type(_),
            )?,
            RelationKind::Returns => self.validate_types(
                "Function",
                "Type",
                RelationSource::Node(_),
                RelationTarget::Type(_),
            )?,
            RelationKind::HasType => self.validate_types(
                "Node",
                "Type",
                RelationSource::Node(_),
                RelationTarget::Type(_),
            )?,
        }
        Ok(())
    }

    fn validate_types(
        &self,
        expected_source: &'static str,
        expected_target: &'static str,
        source_matcher: RelationSource,
        target_matcher: RelationTarget,
    ) -> Result<(), RelationError> {
        if !matches!(self.source, source_matcher) {
            return Err(RelationError::InvalidSourceType {
                kind: self.kind.clone(),
                expected: expected_source,
                found: self.source.type_name(),
            });
        }

        if !matches!(self.target, target_matcher) {
            return Err(RelationError::InvalidTargetType {
                kind: self.kind.clone(),
                expected: expected_target,
                found: self.target.type_name(),
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
// AI
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum RelationSource {
    Node(NodeId),
    Trait(TraitId),
    Type(TypeId),
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
            RelationTarget::Type(id) => GraphNodeId::from(id),
            RelationTarget::Trait(id) => GraphNodeId::from(id),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum RelationTarget {
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
            RelationTarget::Type(_) => "Type",
            RelationTarget::Trait(_) => "Trait",
        }
    }
}
// How many of these From implemenations should we be using here? Are they merited? Why or why not?
// AI?

// ANCHOR: Uses
// Different kinds of relations
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Copy, Clone)]
pub enum RelationKind {
    FunctionParameter,
    FunctionReturn,
    StructField,
    EnumVariant,
    ImplementsFor,
    ImplementsTrait(TraitId), // TraitId type parameter added
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
    // TODO: Consider removing `HasType` later.
    // I don't think it's really useful but am wrestling with bugs rn.
    HasType,
}

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
//ANCHOR_END: Uses
