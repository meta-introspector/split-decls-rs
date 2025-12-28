macro_rules! macro_501 {
    () => {
        crate :: target_spec_enum ! { # [doc = " Which kind of debuginfo does the target use?"] # [doc = ""] # [doc = " Useful in determining whether a target supports Split DWARF (a target with"] # [doc = " `DebuginfoKind::Dwarf` and supporting `SplitDebuginfo::Unpacked` for example)."] # [derive (Default)] pub enum DebuginfoKind { # [doc = " DWARF debuginfo (such as that used on `x86_64_unknown_linux_gnu`)."] # [default] Dwarf = "dwarf" , # [doc = " DWARF debuginfo in dSYM files (such as on Apple platforms)."] DwarfDsym = "dwarf-dsym" , # [doc = " Program database files (such as on Windows)."] Pdb = "pdb" , } parse_error_type = "debuginfo kind" ; }
    };
}

macro_501!()