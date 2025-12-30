// Generated macro for with_segment (function)
macro_rules! Depcrate_filters_pathwith_segment {
() => {
// Module: crate::filters::path
// Provides: {"with_segment"}
// Dependencies: {}
fn with_segment < F , U > (route : & mut Route , func : F) -> Result < U , Rejection > where F : Fn (& str) -> Result < U , Rejection > , { let seg = segment (route) ; let ret = func (seg) ; if ret . is_ok () { let idx = seg . len () ; route . set_unmatched_path (idx) ; } ret }
};
}
