macro_rules! AnnotationKind {
    () => {
        # [derive (PartialEq)] enum AnnotationKind { # [doc = " Annotation is required if not inherited from unstable parents."] Required , # [doc = " Annotation is useless, reject it."] Prohibited , # [doc = " Deprecation annotation is useless, reject it. (Stability attribute is still required.)"] DeprecationProhibited , # [doc = " Annotation itself is useless, but it can be propagated to children."] Container , }
    };
}

AnnotationKind!()