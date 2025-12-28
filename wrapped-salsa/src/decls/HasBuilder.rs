macro_rules! HasBuilder {
    () => {
        pub trait HasBuilder { type Builder ; }
    };
}

HasBuilder!();