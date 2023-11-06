use std::collections::HashMap;

use w3_base::Qcc;

use crate::parser::{profile, slk};

use super::{metadata::Metadata, Object, ObjectKind};

pub struct Store {
    objects: HashMap<Qcc, Object, ahash::RandomState>,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            objects: HashMap::with_hasher(ahash::RandomState::new()),
        }
    }
}

impl Store {
    pub fn insert_slk_row(
        &mut self,
        kind: ObjectKind,
        row: slk::TableRow,
        metadata: &Metadata,
    ) -> Option<()> {
        let has_aliased_id = { row.column_name(0)? == "alias" && row.column_name(1)? == "code" };

        let (id, alias_id) = if has_aliased_id {
            let id = row.by_idx(1)?.as_str()?;
            let alias_id = row.by_idx(0)?.as_str()?;
            (id, Some(alias_id))
        } else {
            let id = row.by_idx(0)?.as_str()?;
            (id, None)
        };

        let id: Qcc = id.parse().ok()?;
        let alias_id: Option<Qcc> = alias_id.and_then(|id| id.parse().ok());

        let object = self
            .objects
            .entry(id)
            .or_insert_with(|| Object::new(id, kind.into()));

        if let Some(alias_id) = alias_id {
            object.aliased_id = Some(alias_id);
        }

        for (name, cell) in row.cells_with_names() {
            object.process_slk_field(cell.value(), name, metadata);
        }

        Some(())
    }

    pub fn insert_func_entry(&mut self, entry: profile::Entry, metadata: &Metadata) -> Option<()> {
        let id = entry.id.parse().ok()?;
        let object = self.objects.get_mut(&id)?;

        for (key, values) in entry.values {
            for (index, value) in values.split(',').enumerate() {
                object.process_func_field(key, value, index as i8, metadata);
            }
        }

        Some(())
    }
}
