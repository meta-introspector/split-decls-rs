macro_rules! deps {
    () => {
        StrLitKind!();
    };
}

macro_rules! find_str_lit_len {
    () => {
        deps!();
        # [doc = " Parses a string literal, returning the byte index of its last character"] # [doc = " (either a quote or a hash)."] fn find_str_lit_len (str_lit_to_eof : & str) -> Option < usize > { fn try_find_n_hashes (s : & mut impl Iterator < Item = char > , desired_hashes : usize ,) -> Option < (usize , Option < char >) > { let mut n = 0 ; loop { match s . next () ? { '#' => n += 1 , c => return Some ((n , Some (c))) , } if n == desired_hashes { return Some ((n , None)) ; } } } let mut s = str_lit_to_eof . chars () ; let kind = match s . next () ? { '"' => StrLitKind :: Normal , 'r' => { let (n , c) = try_find_n_hashes (& mut s , usize :: MAX) ? ; if c != Some ('"') { return None ; } StrLitKind :: Raw (n) } _ => return None , } ; let mut oldc = None ; loop { let c = oldc . take () . or_else (| | s . next ()) ? ; match (c , kind) { ('\\' , StrLitKind :: Normal) => { let _escaped = s . next () ? ; } ('"' , StrLitKind :: Normal) => break , ('"' , StrLitKind :: Raw (0)) => break , ('"' , StrLitKind :: Raw (n)) => { let (seen , c) = try_find_n_hashes (& mut s , n) ? ; if seen == n { break ; } oldc = c ; } _ => { } } } Some (str_lit_to_eof . len () - s . as_str () . len ()) }
    };
}

find_str_lit_len!()