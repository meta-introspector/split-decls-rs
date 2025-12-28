macro_rules! deps {
    () => {
        SeekTarget!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl SeekTarget { fn block (& self) -> BasicBlock { use SeekTarget :: * ; match * self { BlockEntry (block) => block , Early (loc) | After (loc) => loc . block , } } # [doc = " An iterator over all possible `SeekTarget`s in a given block in order, starting with"] # [doc = " `BlockEntry`."] fn iter_in_block (body : & mir :: Body < '_ > , block : BasicBlock) -> impl Iterator < Item = Self > { let statements_and_terminator = (0 ..= body [block] . statements . len ()) . flat_map (| i | (0 .. 2) . map (move | j | (i , j))) . map (move | (i , kind) | { let loc = Location { block , statement_index : i } ; match kind { 0 => SeekTarget :: Early (loc) , 1 => SeekTarget :: After (loc) , _ => unreachable ! () , } }) ; std :: iter :: once (SeekTarget :: BlockEntry (block)) . chain (statements_and_terminator) } }
    };
}

impl_119!();