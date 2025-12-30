// Generated macro for features (function)
macro_rules! Depcrate_detectfeatures {
() => {
// Module: crate::detect
// Provides: {"features"}
// Dependencies: {}
# [doc = " Returns an `Iterator<Item=(&'static str, bool)>` where"] # [doc = " `Item.0` is the feature name, and `Item.1` is a `bool` which"] # [doc = " is `true` if the feature is supported by the host and `false` otherwise."] # [unstable (feature = "stdarch_internal" , issue = "none")] pub fn features () -> impl Iterator < Item = (& 'static str , bool) > { cfg_select ! { any (target_arch = "x86" , target_arch = "x86_64" , target_arch = "arm" , target_arch = "aarch64" , target_arch = "arm64ec" , target_arch = "riscv32" , target_arch = "riscv64" , target_arch = "powerpc" , target_arch = "powerpc64" , target_arch = "mips" , target_arch = "mips64" , target_arch = "loongarch32" , target_arch = "loongarch64" , target_arch = "s390x" ,) => { (0_u8 .. Feature :: _last as u8) . map (| discriminant : u8 | { # [allow (bindings_with_variant_name)] let f : Feature = unsafe { core :: mem :: transmute (discriminant) } ; let name : &'static str = f . to_str () ; let enabled : bool = check_for (f) ; (name , enabled) }) } _ => None . into_iter () , } }
};
}
