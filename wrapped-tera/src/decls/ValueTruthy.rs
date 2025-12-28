macro_rules! ValueTruthy {
    () => {
        pub trait ValueTruthy { fn is_truthy (& self) -> bool ; }
    };
}

ValueTruthy!()