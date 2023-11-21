skillset Robot {
    resource R {
        state { s1 s2 }
        initial s2
        transition {
            s1 -> s2
            s2 -> s1
        }
    }

    event E {
        guard true
        effect {
            R -> s1
        }
    }

    event E {
        guard true
        effect {
            R -> s1
        }
    }
}