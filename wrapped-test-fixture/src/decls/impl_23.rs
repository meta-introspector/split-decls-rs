macro_rules! deps {
    () => {
        MirrorProcMacroExpander!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl ProcMacroExpander for MirrorProcMacroExpander { fn expand (& self , input : & TopSubtree , _ : Option < & TopSubtree > , _ : & Env , _ : Span , _ : Span , _ : Span , _ : String ,) -> Result < TopSubtree , ProcMacroExpansionError > { fn traverse (builder : & mut TopSubtreeBuilder , iter : TtIter < '_ >) { for tt in iter . collect_vec () . into_iter () . rev () { match tt { TtElement :: Leaf (leaf) => builder . push (leaf . clone ()) , TtElement :: Subtree (subtree , subtree_iter) => { builder . open (subtree . delimiter . kind , subtree . delimiter . open) ; traverse (builder , subtree_iter) ; builder . close (subtree . delimiter . close) ; } } } } let mut builder = TopSubtreeBuilder :: new (input . top_subtree () . delimiter) ; traverse (& mut builder , input . iter ()) ; Ok (builder . build ()) } fn eq_dyn (& self , other : & dyn ProcMacroExpander) -> bool { other . type_id () == TypeId :: of :: < Self > () } }
    };
}

impl_23!();