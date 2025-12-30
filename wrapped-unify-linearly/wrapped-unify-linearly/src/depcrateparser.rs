// Generated macro for Parser (trait)
macro_rules! DepcrateParser {
() => {
// Module: crate
// Provides: {"Parser"}
// Dependencies: {}
trait Parser { type Input : Iterator ; type Output ; fn parse (self , input : Self :: Input) -> Result < (Self :: Output , Self :: Input) , () > ; fn chain < P > (self , p : P) -> Chain < Self , P > where Self : Sized { Chain (self , p) } }
};
}
