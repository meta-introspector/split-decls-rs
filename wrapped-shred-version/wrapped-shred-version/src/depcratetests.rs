// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_version_from_hash () { let hash = [0xa5u8 , 0xa5 , 0x5a , 0x5a , 0xa5 , 0xa5 , 0x5a , 0x5a , 0xa5 , 0xa5 , 0x5a , 0x5a , 0xa5 , 0xa5 , 0x5a , 0x5a , 0xa5 , 0xa5 , 0x5a , 0x5a , 0xa5 , 0xa5 , 0x5a , 0x5a , 0xa5 , 0xa5 , 0x5a , 0x5a , 0xa5 , 0xa5 , 0x5a , 0x5a ,] ; let version = version_from_hash (& Hash :: new_from_array (hash)) ; assert_eq ! (version , 1) ; let hash = [0xa5u8 , 0xa5 , 0x5a , 0x5a , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,] ; let version = version_from_hash (& Hash :: new_from_array (hash)) ; assert_eq ! (version , 0xffff) ; let hash = [0xa5u8 , 0xa5 , 0x5a , 0x5a , 0xa5 , 0xa5 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,] ; let version = version_from_hash (& Hash :: new_from_array (hash)) ; assert_eq ! (version , 0x5a5b) ; } # [test] fn test_compute_shred_version () { assert_eq ! (compute_shred_version (& Hash :: default () , None) , 1) ; let mut hard_forks = HardForks :: default () ; assert_eq ! (compute_shred_version (& Hash :: default () , Some (& hard_forks)) , 1) ; hard_forks . register (1) ; assert_eq ! (compute_shred_version (& Hash :: default () , Some (& hard_forks)) , 55551) ; hard_forks . register (1) ; assert_eq ! (compute_shred_version (& Hash :: default () , Some (& hard_forks)) , 46353) ; } }
};
}
