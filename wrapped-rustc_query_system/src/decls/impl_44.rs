macro_rules! impl_44 {
    () => {
        impl From < DepNodeIndex > for QueryInvocationId { # [inline (always)] fn from (dep_node_index : DepNodeIndex) -> Self { QueryInvocationId (dep_node_index . as_u32 ()) } }
    };
}

impl_44!()