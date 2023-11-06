use std::collections::BTreeMap;

use log::warn;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use w3_base::Qcc;

use crate::parser::{profile, slk};

use self::metadata::{FieldMetadata, FieldType, Metadata};

pub mod data;
pub mod metadata;
pub mod store;

/// Trait for different strategies of fetching WC3 built in
/// objects and fields.
pub trait BuiltinProvider {
    fn data(&self, path: &str) -> Option<&[u8]>;

    fn slk<'a>(&'a self, path: &str) -> Option<slk::TableRows<'a>> {
        let table = slk::TableRows::new(self.data(path)?)?;

        Some(table)
    }

    fn profile<'a>(&'a self, path: &str) -> Option<profile::Entries<'a>> {
        let entries = profile::Entries::new(self.data(path)?);

        Some(entries)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Value {
    String(String),
    Int(i32),
    Real(f32),
    Unreal(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ValueKind {
    Int,
    Real,
    Unreal,
    String,
}

impl ValueKind {
    /// Collapse a WC3 data type into a primitive value type.
    ///
    /// Mostly supposed to be used with data types specified in SLKs.
    pub fn new(input: &str) -> Self {
        match input {
            "real" => Self::Real,
            "unreal" => Self::Unreal,
            "int" | "bool" | "attackBits" | "deathType" | "defenseTypeInt" | "detectionType"
            | "teamColor" | "morphFlags" | "silenceFlags" | "stackFlags" | "interactionFlags"
            | "pickFlags" | "versionFlags" | "fullFlags" | "channelType" | "channelFlags"
            | "spellDetail" | "techAvail" => Self::Int,
            _ => Self::String,
        }
    }
}

bitflags::bitflags! {
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
    /// Represents a WC3 object type.
    pub struct ObjectMask: u32 {
        const ABILITY = 0x1;
        const BUFF = 0x2;
        const DESTRUCTABLE = 0x4;
        const MISC = 0x8;
        const UNIT = 0x10;
        const UPGRADE = 0x20;
        const ITEM = 0x40;
        const DOODAD = 0x80;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum ObjectKind {
    Ability,
    Buff,
    Destructable,
    Misc,
    Unit,
    Upgrade,
    Item,
    Doodad,
}

impl ObjectKind {
    pub fn from_flag(flag: ObjectMask) -> Option<Self> {
        match flag {
            ObjectMask::ABILITY => Some(ObjectKind::Ability),
            ObjectMask::BUFF => Some(ObjectKind::Buff),
            ObjectMask::DESTRUCTABLE => Some(ObjectKind::Destructable),
            ObjectMask::MISC => Some(ObjectKind::Misc),
            ObjectMask::UNIT => Some(ObjectKind::Unit),
            ObjectMask::UPGRADE => Some(ObjectKind::Upgrade),
            ObjectMask::ITEM => Some(ObjectKind::Item),
            ObjectMask::DOODAD => Some(ObjectKind::Doodad),
            _ => None,
        }
    }

    pub fn to_flag(&self) -> ObjectMask {
        match self {
            ObjectKind::Ability => ObjectMask::ABILITY,
            ObjectKind::Buff => ObjectMask::BUFF,
            ObjectKind::Destructable => ObjectMask::DESTRUCTABLE,
            ObjectKind::Misc => ObjectMask::MISC,
            ObjectKind::Unit => ObjectMask::UNIT,
            ObjectKind::Upgrade => ObjectMask::UPGRADE,
            ObjectKind::Item => ObjectMask::ITEM,
            ObjectKind::Doodad => ObjectMask::DOODAD,
        }
    }
}

impl From<ObjectKind> for ObjectMask {
    fn from(kind: ObjectKind) -> Self {
        kind.to_flag()
    }
}

impl Value {
    pub fn default(kind: ValueKind) -> Self {
        match kind {
            ValueKind::Unreal => Value::Unreal(0.0),
            ValueKind::Real => Value::Real(0.0),
            ValueKind::Int => Value::Int(0),
            ValueKind::String => Value::String("".into()),
        }
    }

    pub fn parse(value: &str, ty: ValueKind) -> Option<Self> {
        Some(match ty {
            ValueKind::Unreal => Value::Unreal(value.parse().ok()?),
            ValueKind::Real => Value::Real(value.parse().ok()?),
            ValueKind::Int => Value::Int(value.parse().ok()?),
            ValueKind::String => Value::String(value.into()),
        })
    }

    pub fn kind(&self) -> ValueKind {
        match self {
            Value::Int(..) => ValueKind::Int,
            Value::Real(..) => ValueKind::Real,
            Value::Unreal(..) => ValueKind::Unreal,
            Value::String(..) => ValueKind::String,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LeveledValue {
    pub level: u32,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FieldValue {
    Simple { value: Value },
    Leveled { values: Vec<LeveledValue> },
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Field {
    pub id:   Qcc,
    pub kind: FieldValue,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Object {
    pub mask:       ObjectMask,
    pub id:         Qcc,
    pub aliased_id: Option<Qcc>,
    pub parent_id:  Option<Qcc>,
    pub fields:     BTreeMap<Qcc, Field>,
}

impl Object {
    pub fn new(id: Qcc, kind: ObjectMask) -> Object {
        Object {
            id,
            mask: kind,
            aliased_id: None,
            parent_id: None,
            fields: Default::default(),
        }
    }

    pub fn with_parent(id: Qcc, parent_id: Qcc, kind: ObjectMask) -> Object {
        Object {
            id,
            mask: kind,
            aliased_id: None,
            parent_id: Some(parent_id),
            fields: Default::default(),
        }
    }

    /// Returns the "root" ID of this object
    /// for the purposes of field resolution.
    ///
    /// Returns the aliased ID if it exists,
    /// otherwise the parent ID if it exists,
    /// otherwise the object's own ID.
    pub fn root_id(&self) -> Qcc {
        self.aliased_id.or(self.parent_id).unwrap_or(self.id)
    }

    /// Returns true if this field exists on this object, regardless
    /// of whether it has been overriden for this object or not.
    pub fn has_field(&self, field: &FieldMetadata) -> bool {
        if !field.mask.contains(self.mask) {
            return false;
        }

        if !field.extended.is_empty() {
            let held_by_aliased = self
                .aliased_id
                .map(|id| field.extended.contains(&id))
                .unwrap_or(false);

            let held_by_parent = self
                .parent_id
                .map(|id| field.extended.contains(&id))
                .unwrap_or(false);

            let held_by_self = field.extended.contains(&self.id);

            if !held_by_self && !held_by_aliased && !held_by_parent {
                return false;
            }
        }

        true
    }

    pub fn simple_field(&self, id: Qcc) -> Option<&Value> {
        let field = self.fields.get(&id)?;

        match &field.kind {
            FieldValue::Simple { value } => Some(value),
            _ => None,
        }
    }

    pub fn leveled_field(&self, id: Qcc, level: u32) -> Option<&Value> {
        let field = self.fields.get(&id)?;

        match &field.kind {
            FieldValue::Leveled { values } => values
                .iter()
                .find(|value| value.level == level)
                .map(|value| &value.value),
            _ => None,
        }
    }

    pub fn unset_simple_field(&mut self, id: Qcc) {
        self.fields.remove(&id);
    }

    pub fn unset_leveled_field(&mut self, id: Qcc, level: u32) {
        if let Some(field) = self.fields.get_mut(&id) {
            if let FieldValue::Leveled { values } = &mut field.kind {
                values.retain(|dv| dv.level != level)
            }
        }
    }

    pub fn set_simple_field(&mut self, id: Qcc, value: Value) {
        let kind = FieldValue::Simple { value };
        let field = Field { id, kind };
        self.fields.insert(id, field);
    }

    pub fn set_leveled_field(&mut self, id: Qcc, level: u32, value: Value) {
        let field = self.fields.entry(id).or_insert_with(|| Field {
            id,
            kind: FieldValue::Leveled {
                values: Default::default(),
            },
        });

        match &mut field.kind {
            FieldValue::Simple { .. } => warn!(
                "tried to insert data field {} for object {}, but a simple field {} already exists",
                id, self.id, field.id
            ),
            FieldValue::Leveled { values } => {
                let new_value = LeveledValue { level, value };

                if let Some(value) = values.iter_mut().find(|dv| dv.level == level) {
                    *value = new_value;
                } else {
                    values.push(new_value);
                }
            }
        }
    }

    /// Merges this object's data with another object's data
    /// Doesn't do field-level merging because it's not needed
    /// in our use case. Just override the fields in this object
    /// from the fields in the other.
    pub fn merge_into(&mut self, other: &Object) {
        for (id, field) in &other.fields {
            self.fields.insert(*id, field.clone());
        }
    }

    pub fn process_slk_field(
        &mut self,
        value: &slk::Value,
        field_name: &str,
        metadata: &Metadata,
    ) -> Option<()> {
        let (field_meta, level) = metadata.by_slk(field_name, self.id, self.mask)?;

        let value = Value::parse(value.as_raw()?, field_meta.value_ty)?;
        let field_id = field_meta.id;

        match field_meta.ty {
            FieldType::Normal { .. } => self.set_simple_field(field_id, value),
            FieldType::Leveled { .. } | FieldType::Data { .. } => {
                self.set_leveled_field(field_id, level?, value)
            }
        }

        Some(())
    }

    pub fn process_func_field(
        &mut self,
        field_name: &str,
        value: &str,
        index: i8,
        metadata: &Metadata,
    ) -> Option<()> {
        let (field_meta, level) = metadata.by_profile(field_name, self, index)?;
        let value = Value::parse(value, field_meta.value_ty)?;

        if let Some(level) = level {
            self.set_leveled_field(field_meta.id, level, value)
        } else {
            self.set_simple_field(field_meta.id, value)
        }

        Some(())
    }
}
