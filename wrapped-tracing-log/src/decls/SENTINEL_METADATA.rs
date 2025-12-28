macro_rules! SENTINEL_METADATA {
    () => {
        static SENTINEL_METADATA : tracing_core :: Metadata < 'static > = tracing_core :: Metadata :: new ("log interest cache" , "log" , tracing_core :: Level :: ERROR , None , None , None , tracing_core :: field :: FieldSet :: new (& [] , tracing_core :: identify_callsite ! (& SENTINEL_CALLSITE)) , tracing_core :: metadata :: Kind :: EVENT ,) ;
    };
}

SENTINEL_METADATA!()