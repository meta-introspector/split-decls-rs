macro_rules! deps {
    () => {
        PointerMachina!();
    };
}

macro_rules! dotted_pointer {
    () => {
        deps!();
        # [doc = " Lookups a dotted path in a json value"] # [doc = " contrary to the json slash pointer it's not allowed to begin with a dot"] # [inline] # [must_use] pub fn dotted_pointer < 'a > (value : & 'a Value , pointer : & str) -> Option < & 'a Value > { if pointer . is_empty () { return Some (value) ; } PointerMachina :: new (pointer) . map (| mat | mat . replace ("~1" , "/") . replace ("~0" , "~")) . try_fold (value , | target , token | match target { Value :: Object (map) => map . get (& token) , Value :: Array (list) => parse_index (& token) . and_then (| x | list . get (x)) , _ => None , } ,) }
    };
}

dotted_pointer!()