macro_rules! deps {
    () => {
        WrappingRange!();
        Primitive!();
    };
}

macro_rules! Scalar {
    () => {
        deps!();
        # [doc = " Information about one scalar component of a Rust type."] # [derive (Clone , Copy , PartialEq , Eq , Hash , Debug , Serialize)] pub enum Scalar { Initialized { # [doc = " The primitive type used to represent this value."] value : Primitive , # [doc = " The range that represents valid values."] # [doc = " The range must be valid for the `primitive` size."] valid_range : WrappingRange , } , Union { # [doc = " Unions never have niches, so there is no `valid_range`."] # [doc = " Even for unions, we need to use the correct registers for the kind of"] # [doc = " values inside the union, so we keep the `Primitive` type around."] # [doc = " It is also used to compute the size of the scalar."] value : Primitive , } , }
    };
}

Scalar!();