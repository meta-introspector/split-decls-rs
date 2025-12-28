macro_rules! is_ci {
    () => {
        # [inline (always)] pub const fn is_ci () -> bool { option_env ! ("CI") . is_some () }
    };
}

is_ci!();