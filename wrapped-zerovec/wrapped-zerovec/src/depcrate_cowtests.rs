// Generated macro for tests (module)
macro_rules! Depcrate_cowtests {
() => {
// Module: crate::cow
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: VarZeroCow ; use crate :: ule :: tuplevar :: Tuple3VarULE ; use crate :: vecs :: VarZeroSlice ; # [test] fn test_cow_roundtrip () { type Messy = Tuple3VarULE < str , [u8] , VarZeroSlice < str > > ; let vec = vec ! ["one" , "two" , "three"] ; let messy : VarZeroCow < Messy > = VarZeroCow :: from_encodeable (& ("hello" , & b"g\xFF\xFFdbye" [..] , vec)) ; assert_eq ! (messy . a () , "hello") ; assert_eq ! (messy . b () , b"g\xFF\xFFdbye") ; assert_eq ! (& messy . c () [1] , "two") ; # [cfg (feature = "serde")] { let bincode = bincode :: serialize (& messy) . unwrap () ; let deserialized : VarZeroCow < Messy > = bincode :: deserialize (& bincode) . unwrap () ; assert_eq ! (messy , deserialized , "Single element roundtrips with bincode") ; assert ! (! deserialized . is_owned ()) ; let json = serde_json :: to_string (& messy) . unwrap () ; let deserialized : VarZeroCow < Messy > = serde_json :: from_str (& json) . unwrap () ; assert_eq ! (messy , deserialized , "Single element roundtrips with serde") ; } } struct TwoCows < 'a > { cow1 : VarZeroCow < 'a , str > , cow2 : VarZeroCow < 'a , str > , } # [test] fn test_eyepatch_works () { let mut two = TwoCows { cow1 : VarZeroCow :: new_borrowed ("hello") , cow2 : VarZeroCow :: new_owned ("world" . into ()) , } ; let three = VarZeroCow :: new_borrowed (& * two . cow2) ; two . cow1 = three ; } }
};
}
