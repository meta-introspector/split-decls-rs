macro_rules! deps {
    () => {
        SymbolNamesTest!();
        Kind!();
        TestOutput!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl SymbolNamesTest < '_ > { fn process_attrs (& mut self , def_id : LocalDefId) { let tcx = self . tcx ; for attr in tcx . get_attrs (def_id , SYMBOL_NAME) { let def_id = def_id . to_def_id () ; let instance = Instance :: new_raw (def_id , tcx . erase_and_anonymize_regions (GenericArgs :: identity_for_item (tcx , def_id)) ,) ; let mangled = tcx . symbol_name (instance) ; tcx . dcx () . emit_err (TestOutput { span : attr . span () , kind : Kind :: SymbolName , content : format ! ("{mangled}") , }) ; if let Ok (demangling) = rustc_demangle :: try_demangle (mangled . name) { tcx . dcx () . emit_err (TestOutput { span : attr . span () , kind : Kind :: Demangling , content : format ! ("{demangling}") , }) ; tcx . dcx () . emit_err (TestOutput { span : attr . span () , kind : Kind :: DemanglingAlt , content : format ! ("{demangling:#}") , }) ; } } for attr in tcx . get_attrs (def_id , DEF_PATH) { tcx . dcx () . emit_err (TestOutput { span : attr . span () , kind : Kind :: DefPath , content : with_no_trimmed_paths ! (tcx . def_path_str (def_id)) , }) ; } } }
    };
}

impl_45!()