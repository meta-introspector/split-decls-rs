macro_rules! deps {
    () => {
        Region!();
        InferTy!();
        Interner!();
        AliasTy!();
    };
}

macro_rules! Component {
    () => {
        deps!();
        # [derive_where (Debug ; I : Interner)] pub enum Component < I : Interner > { Region (I :: Region) , Param (I :: ParamTy) , Placeholder (I :: PlaceholderTy) , UnresolvedInferenceVariable (ty :: InferTy) , Alias (ty :: AliasTy < I >) , EscapingAlias (Vec < Component < I > >) , }
    };
}

Component!()