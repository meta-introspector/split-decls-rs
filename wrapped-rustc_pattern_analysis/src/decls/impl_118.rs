macro_rules! deps {
    () => {
        WitnessStack!();
        WitnessPat!();
        Constructor!();
        PatCx!();
        PlaceCtxt!();
        WitnessMatrix!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < Cx : PatCx > WitnessMatrix < Cx > { # [doc = " New matrix with no witnesses."] fn empty () -> Self { WitnessMatrix (Vec :: new ()) } # [doc = " New matrix with one `()` witness, i.e. with no columns."] fn unit_witness () -> Self { WitnessMatrix (vec ! [WitnessStack (Vec :: new ())]) } # [doc = " Whether this has any witnesses."] fn is_empty (& self) -> bool { self . 0 . is_empty () } # [doc = " Asserts that there is a single column and returns the patterns in it."] fn single_column (self) -> Vec < WitnessPat < Cx > > { self . 0 . into_iter () . map (| w | w . single_pattern ()) . collect () } # [doc = " Reverses specialization by the `Missing` constructor by pushing a whole new pattern."] fn push_pattern (& mut self , pat : WitnessPat < Cx >) { for witness in self . 0 . iter_mut () { witness . push_pattern (pat . clone ()) } } # [doc = " Reverses specialization by `ctor`. See the section on `unspecialize` at the top of the file."] fn apply_constructor (& mut self , pcx : & PlaceCtxt < '_ , Cx > , missing_ctors : & [Constructor < Cx >] , ctor : & Constructor < Cx > ,) { if self . is_empty () || matches ! (ctor , Constructor :: Or) { return ; } if matches ! (ctor , Constructor :: Missing) { let mut ret = Self :: empty () ; for ctor in missing_ctors { let pat = pcx . wild_from_ctor (ctor . clone ()) ; let mut wit_matrix = self . clone () ; wit_matrix . push_pattern (pat) ; ret . extend (wit_matrix) ; } * self = ret ; } else { for witness in std :: mem :: take (& mut self . 0) { self . 0 . extend (witness . apply_constructor (pcx , ctor)) ; } } } # [doc = " Merges the witnesses of two matrices. Their column types must match."] fn extend (& mut self , other : Self) { self . 0 . extend (other . 0) } }
    };
}

impl_118!()