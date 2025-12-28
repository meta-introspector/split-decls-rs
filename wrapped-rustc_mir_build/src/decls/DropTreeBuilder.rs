macro_rules! deps {
    () => {
        CFG!();
        DropTree!();
    };
}

macro_rules! DropTreeBuilder {
    () => {
        deps!();
        # [doc = " A trait that determined how [DropTree] creates its blocks and"] # [doc = " links to any entry nodes."] trait DropTreeBuilder < 'tcx > { # [doc = " Create a new block for the tree. This should call either"] # [doc = " `cfg.start_new_block()` or `cfg.start_new_cleanup_block()`."] fn make_block (cfg : & mut CFG < 'tcx >) -> BasicBlock ; # [doc = " Links a block outside the drop tree, `from`, to the block `to` inside"] # [doc = " the drop tree."] fn link_entry_point (cfg : & mut CFG < 'tcx > , from : BasicBlock , to : BasicBlock) ; }
    };
}

DropTreeBuilder!();