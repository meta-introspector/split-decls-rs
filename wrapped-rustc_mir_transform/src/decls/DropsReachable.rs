macro_rules! deps {
    () => {
        MovePathIndexAtBlock!();
    };
}

macro_rules! DropsReachable {
    () => {
        deps!();
        struct DropsReachable < 'a , 'mir , 'tcx > { body : & 'a Body < 'tcx > , place : & 'a Place < 'tcx > , drop_span : & 'a mut Option < Span > , move_data : & 'a MoveData < 'tcx > , maybe_init : & 'a mut ResultsCursor < 'mir , 'tcx , MaybeInitializedPlaces < 'mir , 'tcx > > , block_drop_value_info : & 'a mut IndexSlice < BasicBlock , MovePathIndexAtBlock > , collected_drops : & 'a mut MixedBitSet < MovePathIndex > , visited : FxHashMap < BasicBlock , Rc < RefCell < MixedBitSet < MovePathIndex > > > > , }
    };
}

DropsReachable!();