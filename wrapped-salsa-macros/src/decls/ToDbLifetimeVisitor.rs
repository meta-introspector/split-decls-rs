macro_rules! ToDbLifetimeVisitor {
    () => {
        struct ToDbLifetimeVisitor { db_lifetime : syn :: Lifetime , }
    };
}

ToDbLifetimeVisitor!();