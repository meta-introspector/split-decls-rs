macro_rules! SquareRoot {
    () => {
        # [doc = " A **type operator** for taking the integer square root of `Self`."] # [doc = ""] # [doc = " The integer square root of `n` is the largest integer `m` such"] # [doc = " that `n >= m*m`. This definition is equivalent to truncating the"] # [doc = " real-valued square root: `floor(real_sqrt(n))`."] pub trait SquareRoot { # [doc = " The result of the integer square root."] type Output ; }
    };
}

SquareRoot!()