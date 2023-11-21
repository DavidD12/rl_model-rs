skillset S {
    resource R {
        state {A B}
        initial A
        transition {
            A -> B
        }
    }

    event E {
        guard R == B
        effect R -> A
    }
}