skillset Robot<Position, Percent> {

    //-------------------- Data --------------------

    data {
        current: Position
    }

    //-------------------- Resource --------------------

    resource Motor {
        state { On Off }
        initial Off
        transition all
    }

    //-------------------- Data --------------------

    event motor_on {
        guard Motor == Off
        effect Motor -> On
    }

    event motor_off {
        effect Motor -> Off
    }

    //-------------------- Skill --------------------

    skill goto {
        input {
            target: Position
        }
        output {
            final: Position
        }

        precondition {
            motors_on : Motor == On
        }

        start {
            Motor -> On
        }

        invariant {
            motors_on {
                guard Motor == On  // TODO Remove guard 
                effect Motor -> Off
            }
        }

        progress {
            period 1 sec
            message {
                percent: Percent
            }
        }

        interrupt {
            postcondition {
                motors_on: Motor == On
            }
            effect {
                Motor -> Off
            }
        }

        success ok {
            postcondition motor_on: Motor == On
            effect Motor -> Off
        }

        failure ko {
            postcondition motor_on: Motor == On
            effect Motor -> Off
        }
    }

}