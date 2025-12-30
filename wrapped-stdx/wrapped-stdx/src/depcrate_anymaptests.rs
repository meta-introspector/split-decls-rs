// Generated macro for tests (module)
macro_rules! Depcrate_anymaptests {
() => {
// Module: crate::anymap
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_varieties () { fn assert_send < T : Send > () { } fn assert_sync < T : Sync > () { } fn assert_debug < T : :: core :: fmt :: Debug > () { } assert_send :: < Map < dyn Any + Send > > () ; assert_send :: < Map < dyn Any + Send + Sync > > () ; assert_sync :: < Map < dyn Any + Send + Sync > > () ; assert_debug :: < Map < dyn Any > > () ; assert_debug :: < Map < dyn Any + Send > > () ; assert_debug :: < Map < dyn Any + Send + Sync > > () ; } # [test] fn type_id_hasher () { use core :: any :: TypeId ; use core :: hash :: Hash as _ ; fn verify_hashing_with (type_id : TypeId) { let mut hasher = TypeIdHasher :: default () ; type_id . hash (& mut hasher) ; _ = hasher . finish () ; } verify_hashing_with (TypeId :: of :: < usize > ()) ; verify_hashing_with (TypeId :: of :: < () > ()) ; verify_hashing_with (TypeId :: of :: < str > ()) ; verify_hashing_with (TypeId :: of :: < & str > ()) ; verify_hashing_with (TypeId :: of :: < Vec < u8 > > ()) ; } }
};
}
