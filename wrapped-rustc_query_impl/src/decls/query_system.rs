macro_rules! query_system {
    () => {
        pub fn query_system < 'a > (local_providers : Providers , extern_providers : ExternProviders , on_disk_cache : Option < OnDiskCache > , incremental : bool ,) -> QuerySystem < 'a > { QuerySystem { states : Default :: default () , arenas : Default :: default () , caches : Default :: default () , dynamic_queries : dynamic_queries () , on_disk_cache , fns : QuerySystemFns { engine : engine (incremental) , local_providers , extern_providers , encode_query_results : encode_all_query_results , try_mark_green , } , jobs : AtomicU64 :: new (1) , } }
    };
}

query_system!()