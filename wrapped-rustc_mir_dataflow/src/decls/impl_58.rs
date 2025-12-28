macro_rules! deps {
    () => {
        RequiresAnArgument!();
        PathMustEndInFilename!();
        RustcMirAttrs!();
        UnknownFormatter!();
        DuplicateValuesFor!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl RustcMirAttrs { fn parse (tcx : TyCtxt < '_ > , def_id : DefId) -> Result < Self , () > { let mut result = Ok (()) ; let mut ret = RustcMirAttrs :: default () ; let rustc_mir_attrs = tcx . get_attrs (def_id , sym :: rustc_mir) . flat_map (| attr | attr . meta_item_list () . into_iter () . flat_map (| v | v . into_iter ())) ; for attr in rustc_mir_attrs { let attr_result = match attr . name () { Some (name @ sym :: borrowck_graphviz_postflow) => { Self :: set_field (& mut ret . basename_and_suffix , tcx , name , & attr , | s | { let path = PathBuf :: from (s . to_string ()) ; match path . file_name () { Some (_) => Ok (path) , None => { tcx . dcx () . emit_err (PathMustEndInFilename { span : attr . span () }) ; Err (()) } } }) } Some (name @ sym :: borrowck_graphviz_format) => { Self :: set_field (& mut ret . formatter , tcx , name , & attr , | s | match s { sym :: two_phase => Ok (s) , _ => { tcx . dcx () . emit_err (UnknownFormatter { span : attr . span () }) ; Err (()) } }) } _ => Ok (()) , } ; result = result . and (attr_result) ; } result . map (| () | ret) } fn set_field < T > (field : & mut Option < T > , tcx : TyCtxt < '_ > , name : Symbol , attr : & ast :: MetaItemInner , mapper : impl FnOnce (Symbol) -> Result < T , () > ,) -> Result < () , () > { if field . is_some () { tcx . dcx () . emit_err (DuplicateValuesFor { span : attr . span () , name }) ; return Err (()) ; } if let Some (s) = attr . value_str () { * field = Some (mapper (s) ?) ; Ok (()) } else { tcx . dcx () . emit_err (RequiresAnArgument { span : attr . span () , name : attr . name () . unwrap () }) ; Err (()) } } # [doc = " Returns the path where dataflow results should be written, or `None`"] # [doc = " `borrowck_graphviz_postflow` was not specified."] # [doc = ""] # [doc = " This performs the following transformation to the argument of `borrowck_graphviz_postflow`:"] # [doc = ""] # [doc = " \"path/suffix.dot\" -> \"path/analysis_name_suffix.dot\""] fn output_path (& self , analysis_name : & str) -> Option < PathBuf > { let mut ret = self . basename_and_suffix . as_ref () . cloned () ? ; let suffix = ret . file_name () . unwrap () ; let mut file_name : OsString = analysis_name . into () ; file_name . push ("_") ; file_name . push (suffix) ; ret . set_file_name (file_name) ; Some (ret) } }
    };
}

impl_58!()