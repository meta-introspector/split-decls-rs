// Generated macro for impl_142 (impl)
macro_rules! Depcrate_intrinsicimpl_142 {
() => {
// Module: crate::intrinsic
// Provides: {"impl_142"}
// Dependencies: {}
impl Constraint { fn variable (& self) -> & str { match self { Constraint :: AnyI32 { variable , .. } | Constraint :: RangeWildstring { variable , .. } | Constraint :: RangeI32 { variable , .. } | Constraint :: SVEMaxElems { variable , .. } | Constraint :: VecMaxElems { variable , .. } => variable , } } pub fn build (& mut self , ctx : & Context) -> context :: Result { if let Self :: RangeWildstring { variable , range : (min , max) , } = self { min . build_acle (ctx . local) ? ; max . build_acle (ctx . local) ? ; let min = min . to_string () ; let max = max . to_string () ; let min : i32 = min . parse () . map_err (| _ | format ! ("the minimum value `{min}` is not a valid number")) ? ; let max : i32 = max . parse () . or_else (| _ | Ok (type_to_size (max . as_str ()))) . map_err (| _ : ParseIntError | { format ! ("the maximum value `{max}` is not a valid number") }) ? ; * self = Self :: RangeI32 { variable : variable . to_owned () , range : SizeMatchable :: Matched (RangeInclusive :: new (min , max)) , } } # [allow (clippy :: collapsible_if)] if let Self :: SVEMaxElems { sve_max_elems_type : ty , .. } | Self :: VecMaxElems { vec_max_elems_type : ty , .. } = self { if let Some (w) = ty . wildcard () { ty . populate_wildcard (ctx . local . provide_type_wildcard (w) ?) ? ; } } if let Self :: RangeI32 { range , .. } = self { range . perform_match (ctx . local) ? ; } let variable = self . variable () ; ctx . local . variables . contains_key (variable) . then_some (()) . ok_or_else (| | format ! ("cannot build constraint, could not find variable {variable}")) } }
};
}
