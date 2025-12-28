macro_rules! compress {
    () => {
        # [inline] fn compress (ancestor : & mut IndexSlice < PreorderIndex , PreorderIndex > , lastlinked : Option < PreorderIndex > , semi : & IndexSlice < PreorderIndex , PreorderIndex > , label : & mut IndexSlice < PreorderIndex , PreorderIndex > , v : PreorderIndex ,) { assert ! (is_processed (v , lastlinked)) ; let mut stack : smallvec :: SmallVec < [_ ; 8] > = smallvec :: smallvec ! [v] ; let mut u = ancestor [v] ; while is_processed (u , lastlinked) { stack . push (u) ; u = ancestor [u] ; } for & [v , u] in stack . array_windows () . rev () { if semi [label [u]] < semi [label [v]] { label [v] = label [u] ; } ancestor [v] = ancestor [u] ; } }
    };
}

compress!()