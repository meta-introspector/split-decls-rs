use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn par_map < I : DynSend , T : IntoIterator < Item = I > , R : DynSend , C : FromIterator < R > > (t : T , map : impl Fn (I) -> R + DynSync + DynSend ,) -> C { parallel_guard (| guard | { if mode :: is_dyn_thread_safe () { let map = FromDyn :: from (map) ; let mut items : Vec < (Option < I > , Option < R >) > = t . into_iter () . map (| i | (Some (i) , None)) . collect () ; par_slice (& mut items , guard , | i | { i . 1 = Some (map (i . 0 . take () . unwrap ())) ; }) ; items . into_iter () . filter_map (| i | i . 1) . collect () } else { t . into_iter () . filter_map (| i | guard . run (| | map (i))) . collect () } }) }
}