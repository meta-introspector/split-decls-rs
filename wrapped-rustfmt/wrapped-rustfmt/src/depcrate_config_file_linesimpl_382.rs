// Generated macro for impl_382 (impl)
macro_rules! Depcrate_config_file_linesimpl_382 {
() => {
// Module: crate::config::file_lines
// Provides: {"impl_382"}
// Dependencies: {}
impl Range { pub fn new (lo : usize , hi : usize) -> Range { Range { lo , hi } } fn is_empty (self) -> bool { self . lo > self . hi } # [allow (dead_code)] fn contains (self , other : Range) -> bool { if other . is_empty () { true } else { ! self . is_empty () && self . lo <= other . lo && self . hi >= other . hi } } fn intersects (self , other : Range) -> bool { if self . is_empty () || other . is_empty () { false } else { (self . lo <= other . hi && other . hi <= self . hi) || (other . lo <= self . hi && self . hi <= other . hi) } } fn adjacent_to (self , other : Range) -> bool { if self . is_empty () || other . is_empty () { false } else { self . hi + 1 == other . lo || other . hi + 1 == self . lo } } # [doc = " Returns a new `Range` with lines from `self` and `other` if they were adjacent or"] # [doc = " intersect; returns `None` otherwise."] fn merge (self , other : Range) -> Option < Range > { if self . adjacent_to (other) || self . intersects (other) { Some (Range :: new (cmp :: min (self . lo , other . lo) , cmp :: max (self . hi , other . hi) ,)) } else { None } } }
};
}
