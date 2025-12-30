// Generated macro for tests (module)
macro_rules! Depcrate_non_niltests {
() => {
// Module: crate::non_nil
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_non_nil_with_option_size () { assert_eq ! (std :: mem :: size_of ::< Option < NonNilUuid >> () , std :: mem :: size_of ::< Uuid > ()) ; } # [test] fn test_non_nil () { let uuid = Uuid :: from_u128 (0x0123456789abcdef0123456789abcdef) ; assert_eq ! (Uuid :: from (NonNilUuid :: try_from (uuid) . unwrap ()) , uuid) ; assert_eq ! (NonNilUuid :: new (uuid) . unwrap () , uuid) ; assert_eq ! (unsafe { NonNilUuid :: new_unchecked (uuid) } , uuid) ; assert ! (NonNilUuid :: try_from (Uuid :: nil ()) . is_err ()) ; assert ! (NonNilUuid :: new (Uuid :: nil ()) . is_none ()) ; } # [test] fn test_non_nil_formatting () { let uuid = Uuid :: from_u128 (0x0123456789abcdef0123456789abcdef) ; let non_nil = NonNilUuid :: try_from (uuid) . unwrap () ; assert_eq ! (format ! ("{uuid}") , format ! ("{non_nil}")) ; assert_eq ! (format ! ("{uuid:?}") , format ! ("{non_nil:?}")) ; } }
};
}
