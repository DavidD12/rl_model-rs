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
        guard R != s1 and R == s2 or not not R==s1 => true
        effect {
            R -> s1
        }
    }
}