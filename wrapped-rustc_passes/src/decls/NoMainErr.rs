macro_rules! NoMainErr {
    () => {
        pub (crate) struct NoMainErr { pub sp : Span , pub crate_name : Symbol , pub has_filename : bool , pub filename : PathBuf , pub file_empty : bool , pub non_main_fns : Vec < Span > , pub main_def_opt : Option < MainDefinition > , pub add_teach_note : bool , }
    };
}

NoMainErr!()