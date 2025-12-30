// Generated macro for tests (module)
macro_rules! Depcrate_syscall_oracletests {
() => {
// Module: crate::syscall_oracle
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_syscall_interceptor_creation () { let interceptor = create_default_syscall_interceptor () ; assert ! (! interceptor . syscall_mappings . is_empty ()) ; assert ! (interceptor . dao_governance) ; } # [test] fn test_oracle_types_generation () { let oracle_code = generate_oracle_types () ; assert ! (! oracle_code . to_string () . is_empty ()) ; } }
};
}
