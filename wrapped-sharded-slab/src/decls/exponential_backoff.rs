macro_rules! exponential_backoff {
    () => {
        # [inline (always)] fn exponential_backoff (exp : & mut usize) { # [doc = " Maximum exponent we can back off to."] const MAX_EXPONENT : usize = 8 ; for _ in 0 .. (1 << * exp) { hint :: spin_loop () ; } if * exp >= MAX_EXPONENT { crate :: sync :: yield_now () ; } else { * exp += 1 ; } }
    };
}

exponential_backoff!();