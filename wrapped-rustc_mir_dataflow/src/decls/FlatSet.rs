macro_rules! FlatSet {
    () => {
        # [doc = " Extends a type `T` with top and bottom elements to make it a partially ordered set in which no"] # [doc = " value of `T` is comparable with any other."] # [doc = ""] # [doc = " A flat set has the following [Hasse diagram]:"] # [doc = ""] # [doc = " ```text"] # [doc = "          top"] # [doc = "  / ... / /  \\ \\ ... \\"] # [doc = " all possible values of `T`"] # [doc = "  \\ ... \\ \\  / / ... /"] # [doc = "         bottom"] # [doc = " ```"] # [doc = ""] # [doc = " [Hasse diagram]: https://en.wikipedia.org/wiki/Hasse_diagram"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum FlatSet < T > { Bottom , Elem (T) , Top , }
    };
}

FlatSet!();