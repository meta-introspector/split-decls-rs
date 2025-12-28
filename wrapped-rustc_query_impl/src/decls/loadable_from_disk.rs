macro_rules! loadable_from_disk {
    () => {
        pub (crate) fn loadable_from_disk < 'tcx > (tcx : TyCtxt < 'tcx > , id : SerializedDepNodeIndex) -> bool { if let Some (cache) = tcx . query_system . on_disk_cache . as_ref () { cache . loadable_from_disk (id) } else { false } }
    };
}

loadable_from_disk!();