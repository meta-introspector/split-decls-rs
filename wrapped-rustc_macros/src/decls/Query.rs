macro_rules! deps {
    () => {
        QueryModifiers!();
    };
}

macro_rules! Query {
    () => {
        deps!();
        # [doc = " A compiler query. `query ... { ... }`"] struct Query { doc_comments : Vec < Attribute > , modifiers : QueryModifiers , name : Ident , key : Pat , arg : Type , result : ReturnType , }
    };
}

Query!()