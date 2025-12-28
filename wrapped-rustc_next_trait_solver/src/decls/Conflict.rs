macro_rules! Conflict {
    () => {
        # [derive (Debug , Copy , Clone)] pub enum Conflict { Upstream , Downstream , }
    };
}

Conflict!()