// Generated macro for arch (module)
macro_rules! Depcratearch {
() => {
// Module: crate
// Provides: {"arch"}
// Dependencies: {}
# [doc = include_str ! ("../../stdarch/crates/core_arch/src/core_arch_docs.md")] # [stable (feature = "simd_arch" , since = "1.27.0")] pub mod arch { # [stable (feature = "simd_arch" , since = "1.27.0")] # [doc (no_inline)] pub use core :: arch :: * ; # [stable (feature = "simd_aarch64" , since = "1.60.0")] pub use std_detect :: is_aarch64_feature_detected ; # [unstable (feature = "stdarch_arm_feature_detection" , issue = "111190")] pub use std_detect :: is_arm_feature_detected ; # [unstable (feature = "is_loongarch_feature_detected" , issue = "117425")] pub use std_detect :: is_loongarch_feature_detected ; # [unstable (feature = "is_riscv_feature_detected" , issue = "111192")] pub use std_detect :: is_riscv_feature_detected ; # [unstable (feature = "stdarch_s390x_feature_detection" , issue = "135413")] pub use std_detect :: is_s390x_feature_detected ; # [stable (feature = "simd_x86" , since = "1.27.0")] pub use std_detect :: is_x86_feature_detected ; # [unstable (feature = "stdarch_mips_feature_detection" , issue = "111188")] pub use std_detect :: { is_mips_feature_detected , is_mips64_feature_detected } ; # [unstable (feature = "stdarch_powerpc_feature_detection" , issue = "111191")] pub use std_detect :: { is_powerpc_feature_detected , is_powerpc64_feature_detected } ; }
};
}
