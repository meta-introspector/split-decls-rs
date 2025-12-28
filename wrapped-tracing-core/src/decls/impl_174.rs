macro_rules! deps {
    () => {
        Field!();
        Iter!();
        ValueSet!();
        Identifier!();
        FieldSet!();
        Callsite!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl FieldSet { # [doc = " Constructs a new `FieldSet` with the given array of field names and callsite."] pub const fn new (names : & 'static [& 'static str] , callsite : callsite :: Identifier) -> Self { Self { names , callsite } } # [doc = " Returns an [`Identifier`] that uniquely identifies the [`Callsite`]"] # [doc = " which defines this set of fields.."] # [doc = ""] # [doc = " [`Identifier`]: super::callsite::Identifier"] # [doc = " [`Callsite`]: super::callsite::Callsite"] # [inline] pub (crate) fn callsite (& self) -> callsite :: Identifier { callsite :: Identifier (self . callsite . 0) } # [doc = " Returns the [`Field`] named `name`, or `None` if no such field exists."] # [doc = ""] # [doc = " [`Field`]: super::Field"] pub fn field < Q : Borrow < str > + ? Sized > (& self , name : & Q) -> Option < Field > { let name = & name . borrow () ; self . names . iter () . position (| f | f == name) . map (| i | Field { i , fields : FieldSet { names : self . names , callsite : self . callsite () , } , }) } # [doc = " Returns `true` if `self` contains the given `field`."] # [doc = ""] # [doc = " <div class=\"example-wrap\" style=\"display:inline-block\">"] # [doc = " <pre class=\"ignore\" style=\"white-space:normal;font:inherit;\">"] # [doc = " <strong>Note</strong>: If <code>field</code> shares a name with a field"] # [doc = " in this <code>FieldSet</code>, but was created by a <code>FieldSet</code>"] # [doc = " with a different callsite, this <code>FieldSet</code> does <em>not</em>"] # [doc = " contain it. This is so that if two separate span callsites define a field"] # [doc = " named \"foo\", the <code>Field</code> corresponding to \"foo\" for each"] # [doc = " of those callsites are not equivalent."] # [doc = " </pre></div>"] pub fn contains (& self , field : & Field) -> bool { field . callsite () == self . callsite () && field . i <= self . len () } # [doc = " Returns an iterator over the `Field`s in this `FieldSet`."] # [inline] pub fn iter (& self) -> Iter { let idxs = 0 .. self . len () ; Iter { idxs , fields : FieldSet { names : self . names , callsite : self . callsite () , } , } } # [doc = " Returns a new `ValueSet` with entries for this `FieldSet`'s values."] # [doc (hidden)] pub fn value_set < 'v , V > (& 'v self , values : & 'v V) -> ValueSet < 'v > where V : ValidLen < 'v > , { ValueSet { fields : self , values : values . borrow () , } } # [doc = " Returns the number of fields in this `FieldSet`."] # [inline] pub fn len (& self) -> usize { self . names . len () } # [doc = " Returns whether or not this `FieldSet` has fields."] # [inline] pub fn is_empty (& self) -> bool { self . names . is_empty () } }
    };
}

impl_174!();