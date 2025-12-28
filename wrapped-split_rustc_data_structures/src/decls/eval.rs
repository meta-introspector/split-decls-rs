macro_rules! eval {
    () => {
        # [doc = " Evaluate the link-eval virtual forest, providing the currently minimum semi"] # [doc = " value for the passed `node` (which may be itself)."] # [doc = ""] # [doc = " This maintains that for every vertex v, `label[v]` is such that:"] # [doc = ""] # [doc = " ```text"] # [doc = " semi[eval(v)] = min { semi[label[u]] | root_in_forest(v) +> u *> v }"] # [doc = " ```"] # [doc = ""] # [doc = " where `+>` is a proper ancestor and `*>` is just an ancestor."] # [inline] fn eval (ancestor : & mut IndexSlice < PreorderIndex , PreorderIndex > , lastlinked : Option < PreorderIndex > , semi : & IndexSlice < PreorderIndex , PreorderIndex > , label : & mut IndexSlice < PreorderIndex , PreorderIndex > , node : PreorderIndex ,) -> PreorderIndex { if is_processed (node , lastlinked) { compress (ancestor , lastlinked , semi , label , node) ; label [node] } else { node } }
    };
}

eval!()