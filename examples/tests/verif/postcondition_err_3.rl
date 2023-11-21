
skillset S {
    resource R {
        state {A B C}
        initial A
        transition {
            A -> B
            A -> C
            B -> C
        }
    }

    skill S {
        precondition p: R == A
        invariant inv {
            guard R != C
        }
        interrupt {
            interrupting false
            effect R -> C
            postcondition R == B
        }
        success s {
            effect R -> C
            postcondition R == B
        }
        failure f {
            effect R -> C
            postcondition R == C
        }
    }
}
