macro_rules! deps {
    () => {
        StateData!();
        HasBottom!();
        TrackElem!();
        Formatter!();
        Map!();
    };
}

macro_rules! debug_with_context_rec {
    () => {
        deps!();
        fn debug_with_context_rec < V : Debug + Eq + HasBottom > (place : PlaceIndex , place_str : & str , new : & StateData < V > , old : Option < & StateData < V > > , map : & Map < '_ > , f : & mut Formatter < '_ > ,) -> std :: fmt :: Result { if let Some (value) = map . places [place] . value_index { match old { None => writeln ! (f , "{}: {:?}" , place_str , new . get (value)) ? , Some (old) => { if new . get (value) != old . get (value) { writeln ! (f , "\u{001f}-{}: {:?}" , place_str , old . get (value)) ? ; writeln ! (f , "\u{001f}+{}: {:?}" , place_str , new . get (value)) ? ; } } } } for child in map . children (place) { let info_elem = map . places [child] . proj_elem . unwrap () ; let child_place_str = match info_elem { TrackElem :: Discriminant => { format ! ("discriminant({place_str})") } TrackElem :: Variant (idx) => { format ! ("({place_str} as {idx:?})") } TrackElem :: Field (field) => { if place_str . starts_with ('*') { format ! ("({}).{}" , place_str , field . index ()) } else { format ! ("{}.{}" , place_str , field . index ()) } } TrackElem :: DerefLen => { format ! ("Len(*{})" , place_str) } } ; debug_with_context_rec (child , & child_place_str , new , old , map , f) ? ; } Ok (()) }
    };
}

debug_with_context_rec!()