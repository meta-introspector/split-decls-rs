// Generated macro for tests (module)
macro_rules! Depcrate_cfgtests {
() => {
// Module: crate::cfg
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: test_util ; use crate :: Slab ; # [test] # [cfg_attr (loom , ignore)] # [should_panic] fn validates_max_refs () { struct GiantGenConfig ; impl Config for GiantGenConfig { const INITIAL_PAGE_SIZE : usize = 1 ; const MAX_THREADS : usize = 1 ; const MAX_PAGES : usize = 1 ; } let _slab = Slab :: < usize > :: new_with_config :: < GiantGenConfig > () ; } # [test] # [cfg_attr (loom , ignore)] fn big () { let slab = Slab :: new () ; for i in 0 .. 10000 { println ! ("{:?}" , i) ; let k = slab . insert (i) . expect ("insert") ; assert_eq ! (slab . get (k) . expect ("get") , i) ; } } # [test] # [cfg_attr (loom , ignore)] fn custom_page_sz () { let slab = Slab :: new_with_config :: < test_util :: TinyConfig > () ; for i in 0 .. 4096 { println ! ("{}" , i) ; let k = slab . insert (i) . expect ("insert") ; assert_eq ! (slab . get (k) . expect ("get") , i) ; } } }
};
}
