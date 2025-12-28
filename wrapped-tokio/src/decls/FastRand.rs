macro_rules! FastRand {
    () => {
        # [doc = " Fast random number generate."] # [doc = ""] # [doc = " Implement `xorshift64+`: 2 32-bit `xorshift` sequences added together."] # [doc = " Shift triplet `[17,7,16]` was calculated as indicated in Marsaglia's"] # [doc = " `Xorshift` paper: <https://www.jstatsoft.org/article/view/v008i14/xorshift.pdf>"] # [doc = " This generator passes the SmallCrush suite, part of TestU01 framework:"] # [doc = " <http://simul.iro.umontreal.ca/testu01/tu01.html>"] # [derive (Clone , Copy , Debug)] pub (crate) struct FastRand { one : u32 , two : u32 , }
    };
}

FastRand!()