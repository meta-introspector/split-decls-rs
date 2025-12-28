macro_rules! deps {
    () => {
        SyntaxError!();
    };
}

macro_rules! SyntaxTreeBuilder {
    () => {
        deps!();
        # [derive (Default)] pub struct SyntaxTreeBuilder { errors : Vec < SyntaxError > , inner : GreenNodeBuilder < 'static > , }
    };
}

SyntaxTreeBuilder!();