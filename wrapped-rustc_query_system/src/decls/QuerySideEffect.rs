macro_rules! QuerySideEffect {
    () => {
        # [doc = " Tracks 'side effects' for a particular query."] # [doc = " This struct is saved to disk along with the query result,"] # [doc = " and loaded from disk if we mark the query as green."] # [doc = " This allows us to 'replay' changes to global state"] # [doc = " that would otherwise only occur if we actually"] # [doc = " executed the query method."] # [doc = ""] # [doc = " Each side effect gets an unique dep node index which is added"] # [doc = " as a dependency of the query which had the effect."] # [derive (Debug , Encodable , Decodable)] pub enum QuerySideEffect { # [doc = " Stores a diagnostic emitted during query execution."] # [doc = " This diagnostic will be re-emitted if we mark"] # [doc = " the query as green, as that query will have the side"] # [doc = " effect dep node as a dependency."] Diagnostic (DiagInner) , }
    };
}

QuerySideEffect!();