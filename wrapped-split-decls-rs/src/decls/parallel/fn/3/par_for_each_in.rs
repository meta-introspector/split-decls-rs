use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn par_for_each_in < I : DynSend , T : IntoIterator < Item = I > > (t : T , for_each : impl Fn (& I) + DynSync + DynSend ,) { parallel_guard (| guard | { if mode :: is_dyn_thread_safe () { let mut items : Vec < _ > = t . into_iter () . collect () ; par_slice (& mut items , guard , | i | for_each (& * i)) } else { t . into_iter () . for_each (| i | { guard . run (| | for_each (& i)) ; }) ; } }) ; }
}