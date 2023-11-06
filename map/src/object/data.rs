use include_dir::Dir;

use super::BuiltinProvider;

pub mod v1292 {
    use include_dir::{include_dir, Dir};

    pub static DATA: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/object/builtin/v1292");
}

pub struct EmbeddedBuiltinProvider {
    dir: Dir<'static>,
}

impl BuiltinProvider for EmbeddedBuiltinProvider {
    fn data(&self, path: &str) -> Option<&[u8]> {
        self.dir.get_file(path).map(|f| f.contents())
    }
}
