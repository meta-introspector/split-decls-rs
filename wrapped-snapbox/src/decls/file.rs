macro_rules! deps {
    () => {
        DataFormat!();
        Data!();
    };
}

macro_rules! file {
    () => {
        deps!();
        # [doc = " Declare an expected value for an assert from a file"] # [doc = ""] # [doc = " This is relative to the source file the macro is run from"] # [doc = ""] # [doc = " Output type: [`Data`]"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(feature = \"json\")] {"] # [doc = " # use snapbox::file;"] # [doc = " file![\"./test_data/bar.json\"];"] # [doc = " file![\"./test_data/bar.json\": Text];  // do textual rather than structural comparisons"] # [doc = " file![_];"] # [doc = " file![_: Json];  // ensure its treated as json since a type can't be inferred"] # [doc = " # }"] # [doc = " ```"] # [macro_export] macro_rules ! file { [_] => { { let path = $ crate :: data :: generate_snapshot_path ($ crate :: fn_path ! () , None) ; $ crate :: Data :: read_from (& path , None) } } ; [_ : $ type : ident] => { { let format = $ crate :: data :: DataFormat :: $ type ; let path = $ crate :: data :: generate_snapshot_path ($ crate :: fn_path ! () , Some (format)) ; $ crate :: Data :: read_from (& path , Some ($ crate :: data :: DataFormat :: $ type)) } } ; [$ path : literal] => { { let mut path = $ crate :: utils :: current_dir ! () ; path . push ($ path) ; $ crate :: Data :: read_from (& path , None) } } ; [$ path : literal : $ type : ident] => { { let mut path = $ crate :: utils :: current_dir ! () ; path . push ($ path) ; $ crate :: Data :: read_from (& path , Some ($ crate :: data :: DataFormat :: $ type)) } } ; }
    };
}

file!()