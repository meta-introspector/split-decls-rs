macro_rules! deps {
    () => {
        ResultsCursor!();
        ResultsVisitor!();
        Analysis!();
        EffectIndex!();
    };
}

macro_rules! Direction {
    () => {
        deps!();
        pub trait Direction { const IS_FORWARD : bool ; const IS_BACKWARD : bool = ! Self :: IS_FORWARD ; # [doc = " Called by `iterate_to_fixpoint` during initial analysis computation."] fn apply_effects_in_block < 'mir , 'tcx , A > (analysis : & mut A , body : & mir :: Body < 'tcx > , state : & mut A :: Domain , block : BasicBlock , block_data : & 'mir mir :: BasicBlockData < 'tcx > , propagate : impl FnMut (BasicBlock , & A :: Domain) ,) where A : Analysis < 'tcx > ; # [doc = " Called by `ResultsCursor` to recompute the domain value for a location"] # [doc = " in a basic block. Applies all effects between the given `EffectIndex`s."] # [doc = ""] # [doc = " `effects.start()` must precede or equal `effects.end()` in this direction."] fn apply_effects_in_range < 'tcx , A > (analysis : & mut A , state : & mut A :: Domain , block : BasicBlock , block_data : & mir :: BasicBlockData < 'tcx > , effects : RangeInclusive < EffectIndex > ,) where A : Analysis < 'tcx > ; # [doc = " Called by `ResultsVisitor` to recompute the analysis domain values for"] # [doc = " all locations in a basic block (starting from `entry_state` and to"] # [doc = " visit them with `vis`."] fn visit_results_in_block < 'mir , 'tcx , A > (state : & mut A :: Domain , block : BasicBlock , block_data : & 'mir mir :: BasicBlockData < 'tcx > , analysis : & mut A , vis : & mut impl ResultsVisitor < 'tcx , A > ,) where A : Analysis < 'tcx > ; }
    };
}

Direction!()