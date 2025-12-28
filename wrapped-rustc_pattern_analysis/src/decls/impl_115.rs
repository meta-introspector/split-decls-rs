macro_rules! deps {
    () => {
        WitnessStack!();
        PatCx!();
        PlaceCtxt!();
        Constructor!();
        WitnessPat!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < Cx : PatCx > WitnessStack < Cx > { # [doc = " Asserts that the witness contains a single pattern, and returns it."] fn single_pattern (self) -> WitnessPat < Cx > { assert_eq ! (self . 0 . len () , 1) ; self . 0 . into_iter () . next () . unwrap () } # [doc = " Reverses specialization by the `Missing` constructor by pushing a whole new pattern."] fn push_pattern (& mut self , pat : WitnessPat < Cx >) { self . 0 . push (pat) ; } # [doc = " Reverses specialization. Given a witness obtained after specialization, this constructs a"] # [doc = " new witness valid for before specialization. See the section on `unspecialize` at the top of"] # [doc = " the file."] # [doc = ""] # [doc = " Examples:"] # [doc = " ```text"] # [doc = " ctor: tuple of 2 elements"] # [doc = " pats: [false, \"foo\", _, true]"] # [doc = " result: [(false, \"foo\"), _, true]"] # [doc = ""] # [doc = " ctor: Enum::Variant { a: (bool, &'static str), b: usize}"] # [doc = " pats: [(false, \"foo\"), _, true]"] # [doc = " result: [Enum::Variant { a: (false, \"foo\"), b: _ }, true]"] # [doc = " ```"] fn apply_constructor (mut self , pcx : & PlaceCtxt < '_ , Cx > , ctor : & Constructor < Cx > ,) -> SmallVec < [Self ; 1] > { let len = self . 0 . len () ; let arity = pcx . ctor_arity (ctor) ; let fields : Vec < _ > = self . 0 . drain ((len - arity) ..) . rev () . collect () ; if matches ! (ctor , Constructor :: UnionField) && fields . iter () . filter (| p | ! matches ! (p . ctor () , Constructor :: Wildcard)) . count () >= 2 { self . 0 . push (WitnessPat :: wild_from_ctor (pcx . cx , ctor . clone () , pcx . ty . clone ())) ; fields . into_iter () . enumerate () . filter (| (_ , p) | ! matches ! (p . ctor () , Constructor :: Wildcard)) . map (| (i , p) | { let mut ret = self . clone () ; ret . 0 . last_mut () . unwrap () . fields [i] = p ; ret }) . collect () } else { self . 0 . push (WitnessPat :: new (ctor . clone () , fields , pcx . ty . clone ())) ; smallvec ! [self] } } }
    };
}

impl_115!()