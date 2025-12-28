macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! ValueRender {
    () => {
        deps!();
        pub trait ValueRender { fn render (& self , write : & mut impl Write) -> std :: io :: Result < () > ; }
    };
}

ValueRender!();