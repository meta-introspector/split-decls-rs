macro_rules! macro_133 {
    () => {
        declare_lint ! { # [doc = " The `tail_call_track_caller` lint detects usage of `become` attempting to tail call"] # [doc = " a function marked with `#[track_caller]`."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![feature(explicit_tail_calls)]"] # [doc = " #![expect(incomplete_features)]"] # [doc = ""] # [doc = " #[track_caller]"] # [doc = " fn f() {}"] # [doc = ""] # [doc = " fn g() {"] # [doc = "     become f();"] # [doc = " }"] # [doc = ""] # [doc = " g();"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Due to implementation details of tail calls and `#[track_caller]` attribute, calls to"] # [doc = " functions marked with `#[track_caller]` cannot become tail calls. As such using `become`"] # [doc = " is no different than a normal call (except for changes in drop order)."] pub TAIL_CALL_TRACK_CALLER , Warn , "detects tail calls of functions marked with `#[track_caller]`" , @ feature_gate = explicit_tail_calls ; }
    };
}

macro_133!();