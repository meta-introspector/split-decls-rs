macro_rules! deps {
    () => {
        Test!();
    };
}

macro_rules! TestKind {
    () => {
        deps!();
        # [doc = " See [`Test`] for more."] # [derive (Clone , Debug , PartialEq)] enum TestKind < 'tcx > { # [doc = " Test what enum variant a value is."] # [doc = ""] # [doc = " The subset of expected variants is not stored here; instead they are"] # [doc = " extracted from the [`TestCase`]s of the candidates participating in the"] # [doc = " test."] Switch { # [doc = " The enum type being tested."] adt_def : ty :: AdtDef < 'tcx > , } , # [doc = " Test what value an integer or `char` has."] # [doc = ""] # [doc = " The test's target values are not stored here; instead they are extracted"] # [doc = " from the [`TestCase`]s of the candidates participating in the test."] SwitchInt , # [doc = " Test whether a `bool` is `true` or `false`."] If , # [doc = " Test for equality with value, possibly after an unsizing coercion to"] # [doc = " `cast_ty`,"] Eq { value : ty :: Value < 'tcx > , cast_ty : Ty < 'tcx > , } , # [doc = " Test whether the value falls within an inclusive or exclusive range."] Range (Arc < PatRange < 'tcx > >) , # [doc = " Test that the length of the slice is `== len` or `>= len`."] Len { len : u64 , op : BinOp } , # [doc = " Call `Deref::deref[_mut]` on the value."] Deref { # [doc = " Temporary to store the result of `deref()`/`deref_mut()`."] temp : Place < 'tcx > , mutability : Mutability , } , # [doc = " Assert unreachability of never patterns."] Never , }
    };
}

TestKind!();