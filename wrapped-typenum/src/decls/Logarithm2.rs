macro_rules! Logarithm2 {
    () => {
        # [doc = " A **type operator** for taking the integer binary logarithm of `Self`."] # [doc = ""] # [doc = " The integer binary logarighm of `n` is the largest integer `m` such"] # [doc = " that `n >= 2^m`. This definition is equivalent to truncating the"] # [doc = " real-valued binary logarithm: `floor(log2(n))`."] pub trait Logarithm2 { # [doc = " The result of the integer binary logarithm."] type Output ; }
    };
}

Logarithm2!()