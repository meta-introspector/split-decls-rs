macro_rules! deps {
    () => {
        FutureIncompatibilityReason!();
        FutureIncompatibleInfo!();
    };
}

macro_rules! macro_71 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `uninhabited_static` lint detects uninhabited statics."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " enum Void {}"] # [doc = " unsafe extern {"] # [doc = "     static EXTERN: Void;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Statics with an uninhabited type can never be initialized, so they are impossible to define."] # [doc = " However, this can be side-stepped with an `extern static`, leading to problems later in the"] # [doc = " compiler which assumes that there are no initialized uninhabited places (such as locals or"] # [doc = " statics). This was accidentally allowed, but is being phased out."] pub UNINHABITED_STATIC , Warn , "uninhabited static" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: FutureReleaseError , reference : "issue #74840 <https://github.com/rust-lang/rust/issues/74840>" , } ; }
    };
}

macro_71!()