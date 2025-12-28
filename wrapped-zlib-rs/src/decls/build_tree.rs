macro_rules! deps {
    () => {
        State!();
        TreeDesc!();
        Heap!();
    };
}

macro_rules! build_tree {
    () => {
        deps!();
        fn build_tree < const N : usize > (state : & mut State , desc : & mut TreeDesc < N >) { let tree = & mut desc . dyn_tree ; let stree = desc . stat_desc . static_tree ; let elements = desc . stat_desc . elems ; let mut heap = Heap :: new () ; let mut max_code = heap . initialize (& mut tree [.. elements]) ; while heap . heap_len < 2 { heap . heap_len += 1 ; let node = if max_code < 2 { max_code += 1 ; max_code } else { 0 } ; debug_assert ! (node >= 0) ; let node = node as usize ; heap . heap [heap . heap_len] = node as u32 ; * tree [node] . freq_mut () = 1 ; heap . depth [node] = 0 ; state . opt_len -= 1 ; if ! stree . is_empty () { state . static_len -= stree [node] . len () as usize ; } } debug_assert ! (max_code >= 0) ; let max_code = max_code as usize ; desc . max_code = max_code ; let mut n = heap . heap_len / 2 ; while n >= 1 { heap . pqdownheap (tree , n) ; n -= 1 ; } heap . construct_huffman_tree (tree , elements) ; let bl_count = gen_bitlen (state , & mut heap , desc) ; gen_codes (& mut desc . dyn_tree , max_code , & bl_count) ; }
    };
}

build_tree!()