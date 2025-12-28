macro_rules! LookupResult {
    () => {
        # [derive (Copy , Clone , Debug)] pub enum LookupResult { Exact (MovePathIndex) , Parent (Option < MovePathIndex >) , }
    };
}

LookupResult!();