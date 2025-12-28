macro_rules! deps {
    () => {
        Name!();
        CargoMessage!();
        CanonicalPath!();
        Test!();
        Stderr!();
        Project!();
        ParsedOutputs!();
        Context!();
    };
}

macro_rules! parse_cargo_json {
    () => {
        deps!();
        fn parse_cargo_json (project : & Project , stdout : & [u8] , path_map : & Map < CanonicalPath , (& Name , & Test) > ,) -> ParsedOutputs { let mut map = Map :: new () ; let mut nonmessage_stdout = String :: new () ; let mut remaining = & * String :: from_utf8_lossy (stdout) ; let mut seen = Set :: new () ; while ! remaining . is_empty () { let Some (begin) = remaining . find ("{\"reason\":") else { break ; } ; let (nonmessage , rest) = remaining . split_at (begin) ; nonmessage_stdout . push_str (nonmessage) ; let len = match rest . find ('\n') { Some (end) => end + 1 , None => rest . len () , } ; let (message , rest) = rest . split_at (len) ; remaining = rest ; if ! seen . insert (message) { continue ; } if let Ok (de) = serde_json :: from_str :: < CargoMessage > (message) { if de . message . level != "failure-note" { let src_path = CanonicalPath :: new (& de . target . src_path) ; let Some ((name , test)) = path_map . get (& src_path) else { continue ; } ; let entry = map . entry (src_path) . or_insert_with (Stderr :: default) ; if de . message . level == "error" { entry . success = false ; } let normalized = normalize :: diagnostics (& de . message . rendered , Context { krate : & name . 0 , source_dir : & project . source_dir , workspace : & project . workspace , input_file : & test . path , target_dir : & project . target_dir , path_dependencies : & project . path_dependencies , } ,) ; entry . stderr . concat (& normalized) ; } } } nonmessage_stdout . push_str (remaining) ; ParsedOutputs { stdout : nonmessage_stdout , stderrs : map , } }
    };
}

parse_cargo_json!()