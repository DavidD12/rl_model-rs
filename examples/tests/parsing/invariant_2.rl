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
        invariant {
            inv1 {
                guard R==s1
                effect R -> s1
            }
            inv2 {
                guard R != s1
                effect {
                    R -> s1
                    R -> s2
                }
            }
        }
    }
}