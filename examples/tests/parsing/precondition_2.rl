skillset Robot {
    resource R {
        state { s1 s2 }
        initial s2
        transition {
            s1 -> s2
            s2 -> s1
        }
    }

    skill S {
        precondition {
            p1: R == s1
            p2: R == s2
        }
    }
}