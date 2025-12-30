// Generated macro for diff_deadline (function)
macro_rules! Depcrate_algorithms_patiencediff_deadline {
() => {
// Module: crate::algorithms::patience
// Provides: {"diff_deadline"}
// Dependencies: {}
# [doc = " Patience diff algorithm with deadline."] # [doc = ""] # [doc = " Diff `old`, between indices `old_range` and `new` between indices `new_range`."] # [doc = ""] # [doc = " This diff is done with an optional deadline that defines the maximal"] # [doc = " execution time permitted before it bails and falls back to an approximation."] pub fn diff_deadline < Old , New , D > (d : & mut D , old : & Old , old_range : Range < usize > , new : & New , new_range : Range < usize > , deadline : Option < Instant > ,) -> Result < () , D :: Error > where Old : Index < usize > + ? Sized , New : Index < usize > + ? Sized , Old :: Output : Hash + Eq , New :: Output : PartialEq < Old :: Output > + Hash + Eq , D : DiffHook , { let old_indexes = unique (old , old_range . clone ()) ; let new_indexes = unique (new , new_range . clone ()) ; let mut d = Replace :: new (Patience { d , old , old_current : old_range . start , old_end : old_range . end , old_indexes : & old_indexes , new , new_current : new_range . start , new_end : new_range . end , new_indexes : & new_indexes , deadline , }) ; myers :: diff_deadline (& mut d , & old_indexes , 0 .. old_indexes . len () , & new_indexes , 0 .. new_indexes . len () , deadline ,) ? ; Ok (()) }
};
}
