use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::Path,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// use serde::{Deserialize, Serialize};
// use slotmap::{new_key_type, DenseSlotMap};
use w3_base::Qcc;

use crate::parser::slk;

use super::{BuiltinProvider, Object, ObjectKind, ObjectMask, ValueKind};

pub const METADATA_FILES: &[(&str, ObjectKind)] = &[
    ("units/UnitMetaData.slk", ObjectKind::Unit),
    ("units/AbilityMetaData.slk", ObjectKind::Ability),
    ("units/abilitybuffMetaData.slk", ObjectKind::Buff),
    ("units/UpgradeMetaData.slk", ObjectKind::Upgrade),
    ("units/DestructableMetaData.slk", ObjectKind::Destructable),
    ("units/MiscMetaData.slk", ObjectKind::Misc),
    ("doodads/DoodadMetaData.slk", ObjectKind::Doodad),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
/// **M**etadata **F**ield **Key**
pub struct MfKey(pub usize);

impl MfKey {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

struct FieldHeader {
    id:         Qcc,
    name:       String,
    ty:         String,
    index:      i8,
    is_profile: bool,
}

impl FieldHeader {
    fn from_row(row: &slk::TableRow<'_, '_>) -> Option<Self> {
        let field_id = row.by_column("ID")?.as_str()?;
        let field_name = row.by_column("field")?.as_str()?.to_string();
        let value_ty = row.by_column("type")?.as_str()?.to_string();
        let index = row.by_column("index")?.as_num().unwrap_or(-1);
        let slk = row.by_column("slk")?.as_str()?;

        let field_id = Qcc::from_slice(field_id.as_bytes()).unwrap();

        Some(FieldHeader {
            id: field_id,
            name: field_name,
            ty: value_ty,
            index,
            is_profile: slk == "Profile",
        })
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FieldMetadata {
    /// Engine's QCC for the field.
    pub id:           Qcc,
    /// This value is only relevant for Profile-defined fields.
    ///
    /// If it is `-1`, then this indicates that the field *may* sometimes accept
    /// comma-delimited lists, although in practice some fields only take a single
    /// value anyway.
    ///
    /// If it is non-negative, then it will take the value with that index
    /// from the list of values under this field's name.
    ///
    /// For example, the fields `abpx` and `abpy` both have the same name `Buttonpos`,
    /// with indices 0 and 1 respectively. In a profile file, they will look like this:
    /// ```
    /// [ACtn]
    /// Buttonpos=0,2
    /// ```
    pub index:        i8,
    /// Field type.
    pub ty:           FieldType,
    /// Primitive value type of the field.
    pub value_ty:     ValueKind,
    /// Fully qualified name of the primitive value type.
    pub value_ty_raw: String,
    /// If non-empty, then this field only exists on the given objects.
    pub extended:     Vec<Qcc>,
    /// The kinds of objects this field exists on.
    pub mask:         ObjectMask,
    /// Whether this field is a profile field, i.e. the game uses
    /// a profile file to define its values.
    pub is_profile:   bool,
}

impl FieldMetadata {
    pub fn is_on_object(&self, mask: ObjectMask, object_id: Qcc) -> bool {
        self.mask.contains(mask) && (self.extended.is_empty() || self.extended.contains(&object_id))
    }

    pub fn matches_index(&self, index: i8) -> bool {
        if self.ty.is_normal() {
            self.index == index || self.index == -1
        } else {
            self.ty.is_leveled() && (self.index == 0)
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]

pub enum FieldType {
    /// Regular field that only has a single value.
    Normal { name: String },
    /// A field can have multiple values depending on the level.
    ///
    /// "Level" is context-dependent, and only applies to abilities, upgrades and doodads.
    Leveled { name: String },
    /// This is a field that exists in the extended data section of abilities. They can also be leveled.
    ///
    /// The idx differentiates this field from other data fields that exist on this object.
    Data { idx: u8 },
}

impl FieldType {
    pub fn name(&self) -> &str {
        match self {
            FieldType::Normal { name } => name,
            FieldType::Leveled { name } => name,
            FieldType::Data { .. } => "data",
        }
    }

    pub fn is_normal(&self) -> bool {
        matches!(self, FieldType::Normal { .. })
    }

    pub fn is_leveled(&self) -> bool {
        matches!(self, FieldType::Leveled { .. } | FieldType::Data { .. })
    }

    pub fn is_data(&self) -> bool {
        matches!(self, FieldType::Data { .. })
    }

    pub fn data_id(&self) -> Option<u8> {
        match self {
            FieldType::Data { idx: id } => Some(*id),
            _ => None,
        }
    }
}

fn split_at_first_digit(s: &str) -> Option<(&str, &str)> {
    s.find(|c: char| c.is_ascii_digit())
        .map(|i| (&s[0..i], &s[i..s.len()]))
}

fn data_char_to_id(n: u8) -> u8 {
    match n {
        b'a' | b'A' => 1,
        b'b' | b'B' => 2,
        b'c' | b'C' => 3,
        b'd' | b'D' => 4,
        b'e' | b'E' => 5,
        b'f' | b'F' => 6,
        b'g' | b'G' => 7,
        b'h' | b'H' => 8,
        b'i' | b'I' => 9,
        b'j' | b'J' => 10,
        _ => panic!("unknown data field id"),
    }
}

#[derive(Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Metadata {
    // primary store for the fields
    // other collections in this struct hold ids into this vector
    fields:          Vec<FieldMetadata>,
    // some fields are only available for specific objects
    // this maps those objects to their extended fields
    extended_fields: BTreeMap<Qcc, Vec<Qcc>>,
    // mapping between field ids and field keys
    qcc_to_mfid:     BTreeMap<Qcc, MfKey>,
    // mapping between field names and field keys
    // multiple fields may be mapped to the same name,
    // namely if they belong to different objects or have different indices
    // so additional filtering must be performed
    name_to_mfid:    BTreeMap<String, Vec<MfKey>>,
}

impl Metadata {
    pub fn with_provider<T: BuiltinProvider>(provider: &T) -> Self {
        let mut metadata = Metadata::default();

        for (path, kind) in METADATA_FILES {
            if let Some(table) = provider.slk(path) {
                metadata.process_slk_table(table, *kind);
            }
        }

        metadata
    }

    pub fn add_field(&mut self, field: FieldMetadata) {
        let id = field.id;
        let name = field.ty.name().to_ascii_lowercase();
        let key = MfKey::new(self.fields.len());
        self.fields.push(field);
        self.qcc_to_mfid.insert(id, key);
        self.name_to_mfid.entry(name).or_default().push(key);
    }

    pub fn process_generic_row(&mut self, row: slk::TableRow, mask: ObjectMask) -> Option<()> {
        let header = FieldHeader::from_row(&row)?;

        let repeat = row.by_column("repeat")?.as_num::<u8>();
        let mut leveled = false;
        if let Some(repeat) = repeat {
            leveled = repeat != 0;
        }

        let ty = if leveled {
            FieldType::Leveled {
                name: header.name.clone(),
            }
        } else {
            FieldType::Normal {
                name: header.name.clone(),
            }
        };

        self.add_field(FieldMetadata {
            id: header.id,
            index: header.index,
            value_ty: ValueKind::new(&header.ty),
            value_ty_raw: header.ty,
            extended: Vec::new(),
            ty,
            mask,
            is_profile: header.is_profile,
        });

        Some(())
    }

    pub fn process_unit_item_row(&mut self, row: slk::TableRow) -> Option<()> {
        let header = FieldHeader::from_row(&row)?;

        let use_unit: u8 = row.by_column("useUnit")?.as_num().unwrap_or(0);
        let use_bld: u8 = row.by_column("useBuilding")?.as_num().unwrap_or(0);
        let use_hero: u8 = row.by_column("useHero")?.as_num().unwrap_or(0);
        let use_item: u8 = row.by_column("useItem")?.as_num().unwrap_or(0);

        let mut mask = ObjectMask::empty();
        if use_item != 0 {
            mask |= ObjectMask::ITEM
        }

        if (use_unit != 0) || (use_bld != 0) || (use_hero != 0) {
            mask |= ObjectMask::UNIT
        }

        let ty = FieldType::Normal {
            name: header.name.clone(),
        };

        self.add_field(FieldMetadata {
            id: header.id,
            index: header.index,
            value_ty: ValueKind::new(&header.ty),
            value_ty_raw: header.ty,
            extended: Vec::new(),
            ty,
            mask,
            is_profile: header.is_profile,
        });

        Some(())
    }

    pub fn process_ability_row(&mut self, row: slk::TableRow) -> Option<()> {
        let header = FieldHeader::from_row(&row)?;

        let repeat = row.by_column("repeat")?.as_num::<u8>();
        let data_id = row.by_column("data")?.as_num::<u8>();
        let use_specific = row.by_column("useSpecific")?.as_str();

        let mut leveled = false;
        if let Some(repeat) = repeat {
            leveled = repeat != 0;
        }

        let ty = if header.name == "Data" {
            FieldType::Data { idx: data_id? }
        } else if leveled {
            FieldType::Leveled {
                name: header.name.clone(),
            }
        } else {
            FieldType::Normal {
                name: header.name.clone(),
            }
        };

        let mut extended = Vec::new();
        if let Some(use_specific) = use_specific {
            for id in use_specific.split(',') {
                if let Some(id) = Qcc::from_slice(id.as_bytes()) {
                    extended.push(id);
                }
            }

            for id in &extended {
                self.extended_fields.entry(*id).or_default().push(header.id);
            }
        }

        self.add_field(FieldMetadata {
            id: header.id,
            value_ty: ValueKind::new(&header.ty),
            value_ty_raw: header.ty,
            ty,
            extended,
            index: header.index,
            mask: ObjectMask::ABILITY,
            is_profile: header.is_profile,
        });

        Some(())
    }

    pub fn process_slk_table(&mut self, mut table: slk::TableRows, kind: ObjectKind) {
        match kind {
            ObjectKind::Unit | ObjectKind::Item => {
                while let Some(row) = table.advance() {
                    self.process_unit_item_row(row);
                }
            }
            ObjectKind::Ability => {
                while let Some(row) = table.advance() {
                    self.process_ability_row(row);
                }
            }
            _ => {
                while let Some(row) = table.advance() {
                    self.process_generic_row(row, kind.into());
                }
            }
        }
    }

    pub fn process_slk_source(&mut self, src: &str, kind: ObjectKind) {
        let table = slk::TableRows::new(src.as_bytes());

        if table.is_none() {
            return;
        }

        let table = table.unwrap();
        self.process_slk_table(table, kind);
    }

    fn find_named_field<F>(&self, name: &str, filter: F) -> Option<&FieldMetadata>
    where
        F: FnMut(&&FieldMetadata) -> bool,
    {
        self.name_to_mfid
            .get(&name.to_ascii_lowercase())
            .and_then(|v| self.find_field(v.iter().copied(), filter))
    }

    fn find_field<I, F>(&self, iter: I, filter: F) -> Option<&FieldMetadata>
    where
        I: Iterator<Item = MfKey>,
        F: FnMut(&&FieldMetadata) -> bool,
    {
        iter.filter_map(|k| self.fields.get(k.0)).find(filter)
    }

    fn find_data_field(&self, object_id: Qcc, data_id: u8) -> Option<&FieldMetadata> {
        let extended = self.extended_fields.get(&object_id)?;
        let extended_mfid = extended
            .iter()
            .filter_map(|id| self.qcc_to_mfid.get(id))
            .copied();

        self.find_field(extended_mfid, |f| match f.ty {
            FieldType::Data { idx } => idx == data_id,
            _ => false,
        })
    }

    pub fn by_qcc(&self, id: Qcc) -> Option<&FieldMetadata> {
        let key = self.qcc_to_mfid.get(&id)?;
        self.fields.get(key.0)
    }

    /// Queries an SLK field by its name and target object.
    /// The object is necessary because the same field name can
    /// resolve to different fields depending on the object.
    pub fn by_slk(
        &self,
        field_name: &str,
        object_id: Qcc,
        mask: ObjectMask,
    ) -> Option<(&FieldMetadata, Option<u32>)> {
        if let Some(field) =
            self.find_named_field(field_name, |f| !f.is_profile && f.mask.contains(mask))
        {
            Some((field, None))
        } else {
            let (name, raw_level) = split_at_first_digit(field_name)?;
            let level: u32 = raw_level.parse().ok()?;

            let field = if name.starts_with("Data") {
                let data_id = data_char_to_id(name.as_bytes()[4]);
                self.find_data_field(object_id, data_id)?
            } else {
                self.find_named_field(name, |f| !f.is_profile && f.mask.contains(mask))?
            };

            Some((field, Some(level)))
        }
    }

    /// Queries a Profile field by it's name and target object.
    /// The object is necessary because the same field name can
    /// resolve to different fields depending on the object.
    pub fn by_profile(
        &self,
        field_name: &str,
        object: &Object,
        index: i8,
    ) -> Option<(&FieldMetadata, Option<u32>)> {
        // TODO: this currently doesn't check the profile section...
        // even though profile fields are used for game constants.
        // but since they aren't tied to a particular object, they need to be queried with a separate method
        let field = self.find_named_field(field_name, |f| {
            f.is_profile && object.has_field(f) && f.matches_index(index)
        })?;

        if field.ty.is_leveled() {
            Some((field, Some(index as u32)))
        } else {
            Some((field, None))
        }
    }

    pub fn by_object(&self, field_id: Qcc, object: &Object) -> Option<&FieldMetadata> {
        let field = self.by_qcc(field_id)?;

        if object.has_field(field) {
            Some(field)
        } else {
            None
        }
    }

    /// Will return an iterator of all fields available for this object,
    /// irrespective of which fields are set on the object iself.
    pub fn all_by_object<'a: 'b, 'b>(
        &'a self,
        object: &'b Object,
    ) -> impl Iterator<Item = &'a FieldMetadata> + 'b {
        self.fields.iter().filter(move |f| object.has_field(f))
    }

    // /// First tries to fetch a Profile/Func field in the format of
    // /// <field_name><index id>, e.g. Buttonpos1 resolving to abpx for units, and Buttonpos2 resolving to abpy,
    // /// if that fails then will try to grab an slk field using the regular approach
    // pub fn query_lua_field(
    //     &self,
    //     object: &Object,
    //     full_name: &str,
    // ) -> Option<(&FieldDesc, Option<u32>)> {
    //     split_by_digits(full_name)
    //         .map(|(name, index)| {
    //             let err_msg = format!("invalid field name '{}': field must not have leading non-numeric characters after digits", full_name);

    //             (
    //                 name,
    //                 index
    //                     .parse()
    //                     .unwrap_or_else(|_| { panic!("{}", err_msg) }),
    //             )
    //         })
    //         .or(Some((full_name, 0)))
    //         .and_then(|(name, index)| self.query_profile_field(name, object, index))
    //         .or_else(|| self.query_slk_field(full_name, object))
    // }
}
