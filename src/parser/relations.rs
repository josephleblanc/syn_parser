use crate::parser::graph_ids::GraphNodeId;
use crate::parser::nodes::{NodeId, TraitId};
use crate::parser::types::TypeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RelationBatch {
    pub relations: Vec<Relation>,
    pub estimated_size: usize,
}

// ANCHOR: Relation
// Represents a relation between nodes
#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    pub source: RelationSource,
    pub target: RelationTarget,
    pub kind: RelationKind,
}
//ANCHOR_END: Relation

#[derive(Debug, thiserror::Error)]
pub enum RelationError {
    #[error("Invalid source type for {kind:?}. Expected {expected}, found {found}")]
    InvalidSourceType {
        kind: RelationKind,
        expected: &'static str,
        found: &'static str,
    },

    #[error("Invalid target type for {kind:?}. Expected {expected}, found {found}")]
    InvalidTargetType {
        kind: RelationKind,
        expected: &'static str,
        found: &'static str,
    },

    #[error("Circular dependency detected in {kind:?} relation")]
    CircularDependency { kind: RelationKind },

    #[error("Missing required trait ID for {kind:?} relation")]
    MissingTraitId { kind: RelationKind },
    // Finish implementing this error type AI!
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
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum RelationSource {
    Node(NodeId),
    Trait(TraitId),
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

impl From<RelationSource> for GraphNodeId {
    fn from(source: RelationSource) -> Self {
        match source {
            RelationSource::Node(id) => GraphNodeId::from(id),
            RelationSource::Trait(id) => GraphNodeId::from(id),
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

impl From<NodeId> for RelationTarget {
    fn from(id: NodeId) -> Self {
        RelationTarget::Node(id)
    }
}

// ANCHOR: Uses
// Different kinds of relations
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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
//ANCHOR_END: Uses
//ANCHOR_END: Relation
