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
        interrupt {
            interrupting false
            effect {
                R -> s1
            }
            postcondition R == s2
        }
    }
}