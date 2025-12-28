macro_rules! fold {
    () => {
        mod fold { # [derive (Clone , Copy)] pub enum Fold { Zero , One (char) , Two (char , char) , Three (char , char , char) , } impl Iterator for Fold { type Item = char ; # [inline] fn next (& mut self) -> Option < char > { match * self { Fold :: Zero => None , Fold :: One (one) => { * self = Fold :: Zero ; Some (one) } Fold :: Two (one , two) => { * self = Fold :: One (two) ; Some (one) } Fold :: Three (one , two , three) => { * self = Fold :: Two (one , two) ; Some (three) } } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { match * self { Fold :: Zero => (0 , Some (0)) , Fold :: One (..) => (1 , Some (1)) , Fold :: Two (..) => (2 , Some (2)) , Fold :: Three (..) => (3 , Some (3)) , } } } impl From < (char ,) > for Fold { # [inline] fn from ((one ,) : (char ,)) -> Fold { Fold :: One (one) } } impl From < (char , char) > for Fold { # [inline] fn from ((one , two) : (char , char)) -> Fold { Fold :: Two (one , two) } } impl From < (char , char , char) > for Fold { # [inline] fn from ((one , two , three) : (char , char , char)) -> Fold { Fold :: Three (one , two , three) } } }
    };
}

fold!()