macro_rules! CastType {
    () => {
        # [doc (hidden)] # [derive (Copy , Clone)] # [cfg_attr (test , derive (Debug))] # [allow (missing_debug_implementations)] pub enum CastType { Prefix , Suffix , }
    };
}

CastType!()