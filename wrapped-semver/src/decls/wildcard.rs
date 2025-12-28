macro_rules! wildcard {
    () => {
        fn wildcard (input : & str) -> Option < (char , & str) > { if let Some (rest) = input . strip_prefix ('*') { Some (('*' , rest)) } else if let Some (rest) = input . strip_prefix ('x') { Some (('x' , rest)) } else if let Some (rest) = input . strip_prefix ('X') { Some (('X' , rest)) } else { None } }
    };
}

wildcard!()