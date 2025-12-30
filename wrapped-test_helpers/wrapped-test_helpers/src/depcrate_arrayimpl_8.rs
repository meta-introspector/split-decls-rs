// Generated macro for impl_8 (impl)
macro_rules! Depcrate_arrayimpl_8 {
() => {
// Module: crate::array
// Provides: {"impl_8"}
// Dependencies: {}
impl < T : ValueTree , const LANES : usize > ValueTree for ArrayValueTree < [T ; LANES] > { type Value = [T :: Value ; LANES] ; fn current (& self) -> Self :: Value { unsafe { # [allow (clippy :: uninit_assumed_init)] let mut value : [MaybeUninit < T :: Value > ; LANES] = MaybeUninit :: uninit () . assume_init () ; for (tree_elem , value_elem) in self . tree . iter () . zip (value . iter_mut ()) { * value_elem = MaybeUninit :: new (tree_elem . current ()) ; } core :: mem :: transmute_copy (& value) } } fn simplify (& mut self) -> bool { while self . shrinker < LANES { if self . tree [self . shrinker] . simplify () { self . last_shrinker = Some (self . shrinker) ; return true ; } else { self . shrinker += 1 ; } } false } fn complicate (& mut self) -> bool { if let Some (shrinker) = self . last_shrinker { self . shrinker = shrinker ; if self . tree [shrinker] . complicate () { true } else { self . last_shrinker = None ; false } } else { false } } }
};
}
