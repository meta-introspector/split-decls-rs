use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn emit_def_diagnostic < 'db > (db : & 'db dyn HirDatabase , acc : & mut Vec < AnyDiagnostic < 'db > > , diag : & DefDiagnostic , edition : Edition ,) { emit_def_diagnostic_ (db , acc , & diag . kind , edition) }
}