// Generated macro for test (module)
macro_rules! Depcrate_itemtest {
() => {
// Module: crate::item
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn find_nothing () { assert ! (ItemSearchOptions :: new () . search () . is_err ()) ; } # [test] fn limit_two () { let results = ItemSearchOptions :: new () . class (ItemClass :: certificate ()) . limit (2) . search () . unwrap () ; assert_eq ! (results . len () , 2) ; } # [test] fn limit_all () { let results = ItemSearchOptions :: new () . class (ItemClass :: certificate ()) . limit (Limit :: All) . search () . unwrap () ; assert ! (results . len () >= 2) ; } }
};
}
