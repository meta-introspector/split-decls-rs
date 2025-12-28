macro_rules! deps {
    () => {
        Expect!();
    };
}

macro_rules! MockSubscriber {
    () => {
        deps!();
        # [doc = " A subscriber which can validate received traces."] # [doc = ""] # [doc = " For a detailed description and examples see the documentation"] # [doc = " for the methods and the [`subscriber`] module."] # [doc = ""] # [doc = " [`subscriber`]: mod@crate::subscriber"] # [derive (Debug)] pub struct MockSubscriber < F : Fn (& Metadata < '_ >) -> bool > { expected : VecDeque < Expect > , max_level : Option < LevelFilter > , filter : F , name : String , }
    };
}

MockSubscriber!();