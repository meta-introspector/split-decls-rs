macro_rules! deps {
    () => {
        MockLayer!();
        Expect!();
    };
}

macro_rules! MockLayerBuilder {
    () => {
        deps!();
        # [doc = " A builder for constructing [`MockLayer`]s."] # [doc = ""] # [doc = " The methods on this builder set expectations which are then"] # [doc = " validated by the constructed [`MockLayer`]."] # [doc = ""] # [doc = " For a detailed description and examples see the documentation"] # [doc = " for the methods and the [`layer`] module."] # [doc = ""] # [doc = " [`layer`]: mod@crate::layer"] # [derive (Debug)] pub struct MockLayerBuilder { expected : VecDeque < Expect > , name : String , }
    };
}

MockLayerBuilder!();