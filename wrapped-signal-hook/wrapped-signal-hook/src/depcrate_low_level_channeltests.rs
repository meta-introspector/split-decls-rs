// Generated macro for tests (module)
macro_rules! Depcrate_low_level_channeltests {
() => {
// Module: crate::low_level::channel
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: sync :: Arc ; use std :: thread ; use super :: * ; # [test] fn new_empty () { let channel = Channel :: < usize > :: new () ; assert ! (channel . recv () . is_none ()) ; assert ! (channel . recv () . is_none ()) ; } # [test] fn pass_value () { let channel = Channel :: new () ; channel . send (42) ; assert_eq ! (42 , channel . recv () . unwrap ()) ; assert ! (channel . recv () . is_none ()) ; } # [test] fn multiple () { let channel = Channel :: new () ; for i in 0 .. 1000 { channel . send (i) ; assert_eq ! (i , channel . recv () . unwrap ()) ; assert ! (channel . recv () . is_none ()) ; } } # [test] fn overflow () { let channel = Channel :: new () ; for i in 0 .. 10 { channel . send (i) ; } for i in 0 .. 5 { assert_eq ! (i , channel . recv () . unwrap ()) ; } assert ! (channel . recv () . is_none ()) ; } # [test] fn multi_thread () { let channel = Arc :: new (Channel :: < usize > :: new ()) ; let sender = thread :: spawn ({ let channel = Arc :: clone (& channel) ; move | | { for i in 0 .. 4 { channel . send (i) ; } } }) ; let mut results = Vec :: new () ; while results . len () < 4 { results . extend (channel . recv ()) ; } assert_eq ! (vec ! [0 , 1 , 2 , 3] , results) ; sender . join () . unwrap () ; } }
};
}
