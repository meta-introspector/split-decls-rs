macro_rules! deps {
    () => {
        SolverDelegate!();
    };
}

macro_rules! SearchGraphDelegate {
    () => {
        deps!();
        # [doc = " This type is never constructed. We only use it to implement `search_graph::Delegate`"] # [doc = " for all types which impl `SolverDelegate` and doing it directly fails in coherence."] pub (super) struct SearchGraphDelegate < D : SolverDelegate > { _marker : PhantomData < D > , }
    };
}

SearchGraphDelegate!()