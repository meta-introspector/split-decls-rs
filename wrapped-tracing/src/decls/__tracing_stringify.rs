macro_rules! __tracing_stringify {
    () => {
        # [doc (hidden)] # [macro_export] macro_rules ! __tracing_stringify { ($ ($ k : ident) .+) => { { const NAME : $ crate :: __macro_support :: FieldName < { $ crate :: __macro_support :: FieldName :: len ($ crate :: __macro_support :: stringify ! ($ ($ k) .+)) } > = $ crate :: __macro_support :: FieldName :: new ($ crate :: __macro_support :: stringify ! ($ ($ k) .+)) ; NAME . as_str () } } ; }
    };
}

__tracing_stringify!()