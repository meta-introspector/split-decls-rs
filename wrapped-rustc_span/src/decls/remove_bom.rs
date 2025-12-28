macro_rules! deps {
    () => {
        NormalizedPos!();
    };
}

macro_rules! remove_bom {
    () => {
        deps!();
        # [doc = " Removes UTF-8 BOM, if any."] fn remove_bom (src : & mut String , normalized_pos : & mut Vec < NormalizedPos >) { if src . starts_with ('\u{feff}') { src . drain (.. 3) ; normalized_pos . push (NormalizedPos { pos : RelativeBytePos (0) , diff : 3 }) ; } }
    };
}

remove_bom!()