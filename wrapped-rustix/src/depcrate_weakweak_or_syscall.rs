// Generated macro for weak_or_syscall (macro)
macro_rules! Depcrate_weakweak_or_syscall {
() => {
// Module: crate::weak
// Provides: {"weak_or_syscall"}
// Dependencies: {}
# [doc = " A combination of `weakcall` and `syscall`. Use the libc function if it's"] # [doc = " available, and fall back to `libc::syscall` otherwise."] macro_rules ! weak_or_syscall { ($ vis : vis fn $ name : ident ($ ($ arg_name : ident : $ t : ty) ,*) via $ sys_name : ident -> $ ret : ty) => ($ vis unsafe fn $ name ($ ($ arg_name : $ t) ,*) -> $ ret { weak ! { fn $ name ($ ($ t) ,*) -> $ ret } if let Some (fun) = $ name . get () { fun ($ ($ arg_name) ,*) } else { syscall ! { fn $ name ($ ($ arg_name : $ t) ,*) via $ sys_name -> $ ret } $ name ($ ($ arg_name) ,*) } }) }
};
}
