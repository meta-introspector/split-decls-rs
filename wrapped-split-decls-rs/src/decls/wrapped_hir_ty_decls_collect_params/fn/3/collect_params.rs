use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Returns unique params for types and consts contained in `value`."] pub fn collect_params < 'db , T > (value : & T) -> Vec < TypeOrConstParamId > where T : ? Sized + rustc_type_ir :: TypeVisitable < DbInterner < 'db > > , { let mut collector = ParamCollector { params : FxHashSet :: default () , } ; value . visit_with (& mut collector) ; Vec :: from_iter (collector . params) }