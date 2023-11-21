
skillset S {
    resource R {
        state {A B C}
        initial A
        transition {
            A -> B
            B -> C
        }
    }

    skill S {
        precondition p: R == A
        start R -> B
        invariant p1 {
            guard R == B
        }
        interrupt {
            interrupting false
            effect R -> C
        }
        success s {
            effect R -> C
        }
        failure f {
            effect R -> C
        }
    }
}