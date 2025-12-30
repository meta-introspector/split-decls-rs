use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: avx2_support_no_cache_x86");
# [target_feature (enable = "xsave")] # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] # [cold] unsafe fn avx2_support_no_cache_x86 () -> bool { # [cfg (target_arch = "x86")] use core :: arch :: x86 :: { __cpuid_count , _xgetbv } ; # [cfg (target_arch = "x86_64")] use core :: arch :: x86_64 :: { __cpuid_count , _xgetbv } ; let xcr0 = _xgetbv (0) ; let os_avx_support = xcr0 & 6 == 6 ; if os_avx_support { let extended_features_ebx = __cpuid_count (7 , 0) . ebx ; let have_avx2 = (extended_features_ebx >> 5) & 1 == 1 ; if have_avx2 { return true ; } } false }
}