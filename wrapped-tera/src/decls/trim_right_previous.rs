macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! trim_right_previous {
    () => {
        deps!();
        macro_rules ! trim_right_previous { ($ vec : expr) => { if let Some (last) = $ vec . pop () { if let Node :: Text (mut s) = last { s = s . trim_end () . to_string () ; if ! s . is_empty () { $ vec . push (Node :: Text (s)) ; } } else { $ vec . push (last) ; } } } ; ($ cond : expr , $ vec : expr) => { if $ cond { trim_right_previous ! ($ vec) ; } } ; }
    };
}

trim_right_previous!()