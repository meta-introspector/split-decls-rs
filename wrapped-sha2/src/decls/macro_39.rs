macro_rules! macro_39 {
    () => {
        cfg_if :: cfg_if ! { if # [cfg (sha2_backend = "soft")] { mod soft ; use soft :: compress ; } else if # [cfg (sha2_backend = "soft-compact")] { mod soft_compact ; use soft_compact :: compress ; } else if # [cfg (any (target_arch = "x86" , target_arch = "x86_64"))] { mod soft ; mod x86_shani ; use x86_shani :: compress ; } else if # [cfg (all (any (target_arch = "riscv32" , target_arch = "riscv64") , sha2_backend = "riscv-zknh"))] { mod riscv_zknh ; mod riscv_zknh_utils ; use riscv_zknh :: compress ; } else if # [cfg (all (any (target_arch = "riscv32" , target_arch = "riscv64") , sha2_backend = "riscv-zknh-compact"))] { mod riscv_zknh_compact ; mod riscv_zknh_utils ; use riscv_zknh_compact :: compress ; } else if # [cfg (target_arch = "aarch64")] { mod soft ; mod aarch64_sha2 ; use aarch64_sha2 :: compress ; } else if # [cfg (target_arch = "loongarch64")] { mod loongarch64_asm ; use loongarch64_asm :: compress ; } else if # [cfg (all (target_arch = "wasm32" , target_feature = "simd128"))] { mod wasm32_simd128 ; use wasm32_simd128 :: compress ; } else { mod soft ; use soft :: compress ; } }
    };
}

macro_39!()