macro_rules! Alt {
    () => {
        # [doc = " A visitor wrapper that ensures any `fmt::Debug` fields are formatted using"] # [doc = " the alternate (`:#`) formatter."] # [derive (Debug , Clone)] pub struct Alt < V > (V) ;
    };
}

Alt!()