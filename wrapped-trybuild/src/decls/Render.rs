macro_rules! Render {
    () => {
        pub (crate) enum Render < 'a > { Common (& 'a str) , Unique (& 'a str) , }
    };
}

Render!()