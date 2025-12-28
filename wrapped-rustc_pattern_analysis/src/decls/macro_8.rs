macro_rules! macro_8 {
    () => {
        # [cfg (feature = "rustc")] rustc_fluent_macro :: fluent_messages ! { "../messages.ftl" }
    };
}

macro_8!()