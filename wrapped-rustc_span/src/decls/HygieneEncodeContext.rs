macro_rules! deps {
    () => {
        ExpnId!();
        SyntaxContextData!();
        SyntaxContext!();
    };
}

macro_rules! HygieneEncodeContext {
    () => {
        deps!();
        # [derive (Default)] pub struct HygieneEncodeContext { # [doc = " All `SyntaxContexts` for which we have written `SyntaxContextData` into crate metadata."] # [doc = " This is `None` after we finish encoding `SyntaxContexts`, to ensure"] # [doc = " that we don't accidentally try to encode any more `SyntaxContexts`"] serialized_ctxts : Lock < FxHashSet < SyntaxContext > > , # [doc = " The `SyntaxContexts` that we have serialized (e.g. as a result of encoding `Spans`)"] # [doc = " in the most recent 'round' of serializing. Serializing `SyntaxContextData`"] # [doc = " may cause us to serialize more `SyntaxContext`s, so serialize in a loop"] # [doc = " until we reach a fixed point."] latest_ctxts : Lock < FxHashSet < SyntaxContext > > , serialized_expns : Lock < FxHashSet < ExpnId > > , latest_expns : Lock < FxHashSet < ExpnId > > , }
    };
}

HygieneEncodeContext!();