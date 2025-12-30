// Generated macro for key (module)
macro_rules! Depcrate_sys_thread_localkey {
() => {
// Module: crate::sys::thread_local
// Provides: {"key"}
// Dependencies: {}
# [doc = " `const`-creatable TLS keys."] # [doc = ""] # [doc = " Most OSs without native TLS will provide a library-based way to create TLS"] # [doc = " storage. For each TLS variable, we create a key, which can then be used to"] # [doc = " reference an entry in a thread-local table. This then associates each key"] # [doc = " with a pointer which we can get and set to store our data."] pub (crate) mod key { cfg_select ! { any (all (not (target_vendor = "apple") , not (target_family = "wasm") , target_family = "unix" ,) , all (not (target_thread_local) , target_vendor = "apple") , target_os = "teeos" , all (target_os = "wasi" , target_env = "p1" , target_feature = "atomics") ,) => { mod racy ; mod unix ; # [cfg (test)] mod tests ; pub (super) use racy :: LazyKey ; pub (super) use unix :: { Key , set } ; # [cfg (any (not (target_thread_local) , test))] pub (super) use unix :: get ; use unix :: { create , destroy } ; } all (not (target_thread_local) , target_os = "windows") => { # [cfg (test)] mod tests ; mod windows ; pub (super) use windows :: { Key , LazyKey , get , run_dtors , set } ; } all (target_vendor = "fortanix" , target_env = "sgx") => { mod racy ; mod sgx ; # [cfg (test)] mod tests ; pub (super) use racy :: LazyKey ; pub (super) use sgx :: { Key , get , set } ; use sgx :: { create , destroy } ; } target_os = "xous" => { mod racy ; # [cfg (test)] mod tests ; mod xous ; pub (super) use racy :: LazyKey ; pub (crate) use xous :: destroy_tls ; pub (super) use xous :: { Key , get , set } ; use xous :: { create , destroy } ; } _ => { } } }
};
}
