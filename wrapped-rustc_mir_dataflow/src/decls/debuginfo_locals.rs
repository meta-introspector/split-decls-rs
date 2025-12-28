macro_rules! deps {
    () => {
        DebuginfoLocals!();
    };
}

macro_rules! debuginfo_locals {
    () => {
        deps!();
        # [doc = " Return the set of locals that appear in debuginfo."] pub fn debuginfo_locals (body : & Body < '_ >) -> DenseBitSet < Local > { let mut visitor = DebuginfoLocals (DenseBitSet :: new_empty (body . local_decls . len ())) ; for debuginfo in body . var_debug_info . iter () { visitor . visit_var_debug_info (debuginfo) ; } visitor . 0 }
    };
}

debuginfo_locals!()