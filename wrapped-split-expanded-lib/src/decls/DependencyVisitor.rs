macro_rules! DependencyVisitor {
    () => {
        struct DependencyVisitor { required_imports : HashSet < String > , _known_identifiers : HashSet < String > , }
    };
}

DependencyVisitor!();