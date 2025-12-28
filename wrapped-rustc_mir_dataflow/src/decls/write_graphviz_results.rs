macro_rules! deps {
    () => {
        Formatter!();
        RustcMirAttrs!();
        DebugWithContext!();
        Analysis!();
        OutputStyle!();
        Results!();
    };
}

macro_rules! write_graphviz_results {
    () => {
        deps!();
        # [doc = " Writes a DOT file containing the results of a dataflow analysis if the user requested it via"] # [doc = " `rustc_mir` attributes and `-Z dump-mir-dataflow`. The `Result` in and the `Results` out are"] # [doc = " the same."] pub (super) fn write_graphviz_results < 'tcx , A > (tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , analysis : & mut A , results : & Results < A :: Domain > , pass_name : Option < & 'static str > ,) -> std :: io :: Result < () > where A : Analysis < 'tcx > , A :: Domain : DebugWithContext < A > , { use std :: fs ; use std :: io :: Write ; let def_id = body . source . def_id () ; let Ok (attrs) = RustcMirAttrs :: parse (tcx , def_id) else { return Ok (()) ; } ; let file = try { match attrs . output_path (A :: NAME) { Some (path) => { debug ! ("printing dataflow results for {:?} to {}" , def_id , path . display ()) ; if let Some (parent) = path . parent () { fs :: create_dir_all (parent) ? ; } fs :: File :: create_buffered (& path) ? } None => { let Some (dumper) = MirDumper :: new (tcx , A :: NAME , body) else { return Ok (()) ; } ; let disambiguator = & pass_name . unwrap_or ("-----") ; dumper . set_disambiguator (disambiguator) . create_dump_file ("dot" , body) ? } } } ; let mut file = match file { Ok (f) => f , Err (e) => return Err (e) , } ; let style = match attrs . formatter { Some (sym :: two_phase) => OutputStyle :: BeforeAndAfter , _ => OutputStyle :: AfterOnly , } ; let mut buf = Vec :: new () ; let graphviz = Formatter :: new (body , analysis , results , style) ; let mut render_opts = vec ! [dot :: RenderOption :: Fontname (tcx . sess . opts . unstable_opts . graphviz_font . clone ())] ; if tcx . sess . opts . unstable_opts . graphviz_dark_mode { render_opts . push (dot :: RenderOption :: DarkTheme) ; } let r = with_no_trimmed_paths ! (dot :: render_opts (& graphviz , & mut buf , & render_opts)) ; let lhs = try { r ? ; file . write_all (& buf) ? ; } ; lhs }
    };
}

write_graphviz_results!();