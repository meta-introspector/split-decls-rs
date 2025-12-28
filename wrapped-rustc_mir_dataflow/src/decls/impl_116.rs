macro_rules! deps {
    () => {
        MockAnalysis!();
        EffectIndex!();
        Direction!();
        Effect!();
        SeekTarget!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < D : Direction > MockAnalysis < '_ , D > { const BASIC_BLOCK_OFFSET : usize = 100 ; # [doc = " The entry set for each `BasicBlock` is the ID of that block offset by a fixed amount to"] # [doc = " avoid colliding with the statement/terminator effects."] fn mock_entry_set (& self , bb : BasicBlock) -> DenseBitSet < usize > { let mut ret = self . bottom_value (self . body) ; ret . insert (Self :: BASIC_BLOCK_OFFSET + bb . index ()) ; ret } fn mock_results (& self) -> IndexVec < BasicBlock , DenseBitSet < usize > > { let empty = self . bottom_value (self . body) ; let mut ret = IndexVec :: from_elem (empty , & self . body . basic_blocks) ; for (bb , _) in self . body . basic_blocks . iter_enumerated () { ret [bb] = self . mock_entry_set (bb) ; } ret } # [doc = " Returns the index that should be added to the dataflow state at the given target."] fn effect (& self , loc : EffectIndex) -> usize { let idx = match loc . effect { Effect :: Early => loc . statement_index * 2 , Effect :: Primary => loc . statement_index * 2 + 1 , } ; assert ! (idx < Self :: BASIC_BLOCK_OFFSET , "Too many statements in basic block") ; idx } # [doc = " Returns the expected state at the given `SeekTarget`."] # [doc = ""] # [doc = " This is the union of index of the target basic block, the index assigned to the"] # [doc = " target statement or terminator, and the indices of all preceding statements in the target"] # [doc = " basic block."] # [doc = ""] # [doc = " For example, the expected state when calling"] # [doc = " `seek_before_primary_effect(Location { block: 2, statement_index: 2 })`"] # [doc = " would be `[102, 0, 1, 2, 3, 4]`."] fn expected_state_at_target (& self , target : SeekTarget) -> DenseBitSet < usize > { let block = target . block () ; let mut ret = self . bottom_value (self . body) ; ret . insert (Self :: BASIC_BLOCK_OFFSET + block . index ()) ; let target = match target { SeekTarget :: BlockEntry { .. } => return ret , SeekTarget :: Early (loc) => Effect :: Early . at_index (loc . statement_index) , SeekTarget :: After (loc) => Effect :: Primary . at_index (loc . statement_index) , } ; let mut pos = if D :: IS_FORWARD { Effect :: Early . at_index (0) } else { Effect :: Early . at_index (self . body [block] . statements . len ()) } ; loop { ret . insert (self . effect (pos)) ; if pos == target { return ret ; } if D :: IS_FORWARD { pos = pos . next_in_forward_order () ; } else { pos = pos . next_in_backward_order () ; } } } }
    };
}

impl_116!()