macro_rules! deps {
    () => {
        Expect!();
        MockLayerBuilder!();
    };
}

macro_rules! MockLayer {
    () => {
        deps!();
        # [doc = " A layer which validates the traces it receives."] # [doc = ""] # [doc = " A `MockLayer` is constructed with a"] # [doc = " [`MockLayerBuilder`]. For a detailed description and examples,"] # [doc = " see the documentation for that struct and for the [`layer`]"] # [doc = " module."] # [doc = ""] # [doc = " [`layer`]: mod@crate::layer"] pub struct MockLayer { expected : Arc < Mutex < VecDeque < Expect > > > , current : Mutex < Vec < Id > > , name : String , }
    };
}

MockLayer!()