macro_rules! deps {
    () => {
        DataFormat!();
    };
}

macro_rules! generate_snapshot_path {
    () => {
        deps!();
        # [doc (hidden)] pub fn generate_snapshot_path (fn_path : & str , format : Option < DataFormat >) -> std :: path :: PathBuf { use std :: fmt :: Write as _ ; let fn_path_normalized = fn_path . replace ("::" , "__") ; let mut path = format ! ("tests/snapshots/{fn_path_normalized}") ; let count = runtime :: get () . count (& path) ; if 0 < count { write ! (& mut path , "@{count}") . unwrap () ; } path . push ('.') ; path . push_str (format . unwrap_or (DataFormat :: Text) . ext ()) ; path . into () }
    };
}

generate_snapshot_path!();