macro_rules! RecompositionState {
    () => {
        # [derive (Clone)] enum RecompositionState { Composing , Purging (usize) , Finished (usize) , }
    };
}

RecompositionState!();