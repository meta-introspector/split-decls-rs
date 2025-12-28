macro_rules! Val {
    () => {
        pub type Val < 'a > = Cow < 'a , Value > ;
    };
}

Val!()