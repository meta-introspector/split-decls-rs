macro_rules! deps {
    () => {
        TokenTree!();
    };
}

macro_rules! TopSubtreeBuilder {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct TopSubtreeBuilder < S > { unclosed_subtree_indices : Vec < usize > , token_trees : Vec < TokenTree < S > > , last_closed_subtree : Option < usize > , }
    };
}

TopSubtreeBuilder!()