macro_rules! deps {
    () => {
        MockAnalysis!();
        Effect!();
        Direction!();
        Analysis!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < 'tcx , D : Direction > Analysis < 'tcx > for MockAnalysis < 'tcx , D > { type Domain = DenseBitSet < usize > ; type Direction = D ; const NAME : & 'static str = "mock" ; fn bottom_value (& self , body : & mir :: Body < 'tcx >) -> Self :: Domain { DenseBitSet :: new_empty (Self :: BASIC_BLOCK_OFFSET + body . basic_blocks . len ()) } fn initialize_start_block (& self , _ : & mir :: Body < 'tcx > , _ : & mut Self :: Domain) { unimplemented ! ("This is never called since `MockAnalysis` is never iterated to fixpoint") ; } fn apply_early_statement_effect (& mut self , state : & mut Self :: Domain , _statement : & mir :: Statement < 'tcx > , location : Location ,) { let idx = self . effect (Effect :: Early . at_index (location . statement_index)) ; assert ! (state . insert (idx)) ; } fn apply_primary_statement_effect (& mut self , state : & mut Self :: Domain , _statement : & mir :: Statement < 'tcx > , location : Location ,) { let idx = self . effect (Effect :: Primary . at_index (location . statement_index)) ; assert ! (state . insert (idx)) ; } fn apply_early_terminator_effect (& mut self , state : & mut Self :: Domain , _terminator : & mir :: Terminator < 'tcx > , location : Location ,) { let idx = self . effect (Effect :: Early . at_index (location . statement_index)) ; assert ! (state . insert (idx)) ; } fn apply_primary_terminator_effect < 'mir > (& mut self , state : & mut Self :: Domain , terminator : & 'mir mir :: Terminator < 'tcx > , location : Location ,) -> TerminatorEdges < 'mir , 'tcx > { let idx = self . effect (Effect :: Primary . at_index (location . statement_index)) ; assert ! (state . insert (idx)) ; terminator . edges () } }
    };
}

impl_117!()