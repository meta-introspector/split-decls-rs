macro_rules! QueryInvocationId {
    () => {
        # [doc = " Something that uniquely identifies a query invocation."] pub struct QueryInvocationId (pub u32) ;
    };
}

QueryInvocationId!()