macro_rules! SubtreeBuilderRestorePoint {
    () => {
        # [derive (Clone , Copy)] pub struct SubtreeBuilderRestorePoint { unclosed_subtree_indices_len : usize , token_trees_len : usize , last_closed_subtree : Option < usize > , }
    };
}

SubtreeBuilderRestorePoint!();