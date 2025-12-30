// Generated macro for tests (module)
macro_rules! Depcrate_unix_freebsd_componenttests {
() => {
// Module: crate::unix::freebsd::component
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: Component ; use crate :: unix :: freebsd :: { ComponentInner , ComponentsInner } ; # [test] fn test_components () { let component1 = Component { inner : ComponentInner :: new (b"dev.cpu.0.temperature\0" . to_vec () , 1.234 , 0) , } ; let component2 = Component { inner : ComponentInner :: new (b"dev.cpu.1.temperature\0" . to_vec () , 5.678 , 1) , } ; assert_eq ! (component1 . id () , Some ("cpu_1")) ; assert_eq ! (component1 . label () , "CPU 1") ; assert_eq ! (component1 . temperature () , Some (1.234)) ; assert_eq ! (component2 . id () , Some ("cpu_2")) ; assert_eq ! (component2 . label () , "CPU 2") ; assert_eq ! (component2 . temperature () , Some (5.678)) ; } }
};
}
