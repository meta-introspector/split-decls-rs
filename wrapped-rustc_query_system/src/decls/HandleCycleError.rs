macro_rules! HandleCycleError {
    () => {
        # [derive (Copy , Clone)] pub enum HandleCycleError { Error , Fatal , DelayBug , Stash , }
    };
}

HandleCycleError!()