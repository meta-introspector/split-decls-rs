// Generated macro for test (module)
macro_rules! Depcrate_unix_linux_cputest {
() => {
// Module: crate::unix::linux::cpu
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: get_vendor_id_and_brand_inner ; # [test] fn test_cpu_retrieval () { const DATA : & str = r#"
processor		: 1
cpu model		: Loongson-3 V0.4  FPU V0.1
model name		: Loongson-3A R4 (Loongson-3B4000) @ 1800MHz
CPU MHz			: 1800.00
core			: 1

processor		: 2
cpu model		: Loongson-3 V0.4  FPU V0.1
model name		: Loongson-3A R4 (Loongson-3B4000) @ 1800MHz
CPU MHz			: 1800.00
package			: 0
core			: 2

processor		: 3
cpu model		: Loongson-3 V0.4  FPU V0.1
model name		: Loongson-3A R4 (Loongson-3B4000) @ 1800MHz
CPU MHz			: 1800.00
core			: 3

processor		: 4
cpu model		: Loongson-3 V0.4  FPU V0.1
model name		: Loongson-3A R4 (Loongson-3B4000) @ 1800MHz
CPU MHz			: 1800.00
core			: 0

processor		: 5
cpu model		: Loongson-3 V0.4  FPU V0.1
model name		: Loongson-3A R4 (Loongson-3B4000) @ 1800MHz
CPU MHz			: 1800.00
core			: 1

processor		: 6
cpu model		: Loongson-3 V0.4  FPU V0.1
model name		: Loongson-3A R4 (Loongson-3B4000) @ 1800MHz
CPU MHz			: 1800.00
core			: 2

processor		: 7
cpu model		: Loongson-3 V0.4  FPU V0.1
model name		: Loongson-3A R4 (Loongson-3B4000) @ 1800MHz
CPU MHz			: 1800.00
core			: 3"# ; let cpus = get_vendor_id_and_brand_inner (DATA) ; assert_eq ! (cpus . len () , 7) ; } }
};
}
