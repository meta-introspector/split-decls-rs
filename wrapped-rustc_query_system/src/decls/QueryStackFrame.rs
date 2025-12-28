macro_rules! deps {
    () => {
        DepKind!();
        QueryStackFrameExtra!();
        QueryStackDeferred!();
    };
}

macro_rules! QueryStackFrame {
    () => {
        deps!();
        # [doc = " Description of a frame in the query stack."] # [doc = ""] # [doc = " This is mostly used in case of cycles for error reporting."] # [derive (Clone , Debug)] pub struct QueryStackFrame < I > { # [doc = " This field initially stores a `QueryStackDeferred` during collection,"] # [doc = " but can later be changed to `QueryStackFrameExtra` containing concrete information"] # [doc = " by calling `lift`. This is done so that collecting query does not need to invoke"] # [doc = " queries, instead `lift` will call queries in a more appropriate location."] pub info : I , pub dep_kind : DepKind , # [doc = " This hash is used to deterministically pick"] # [doc = " a query to remove cycles in the parallel compiler."] hash : Hash64 , pub def_id : Option < DefId > , # [doc = " A def-id that is extracted from a `Ty` in a query key"] pub def_id_for_ty_in_cycle : Option < DefId > , }
    };
}

QueryStackFrame!()