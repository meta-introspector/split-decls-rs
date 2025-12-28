macro_rules! LocalCollector {
    () => {
        # [derive (Default)] struct LocalCollector { locals : FxHashSet < HirId > , }
    };
}

LocalCollector!();