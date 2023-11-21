skillset S {
    resource R {
        state {A B C}
        initial A
        transition {
            A -> B
            A -> C
        }
    }

    skill S {
        precondition p: R == A
        start R -> B
        invariant p1 {
            guard R == B
            effect R -> C
        }
    }
}