type Position
type float

skillset uav {
    resource authority {
        state { Free Pilot Drone }
        initial Pilot
        transition {
            Free  -> Drone
            Free  -> Pilot
            Drone -> Free
            Drone -> Pilot
            Pilot -> Free            
        }
    }

    resource flight_status {
        state { OnGround Hovering Moving }
        initial OnGround
        transition all
    }

    resource axes_authority {
        state { Available Used }
        initial Available
        transition all
    }

    event take_authority {
        guard authority != Pilot
        effect authority -> Pilot
    }
/*
    skill takeoff {
        input {
            height: float
            speed: float
        }
        output {
            height: float
        }
        precondition {
            has_authority : authority==Drone
            on_ground     : flight_status==OnGround
            no_moving     : axes_authority==Available
        }
        start axes_authority -> Used
        invariant {
            keep_authority {
                guard  authority==Drone
                effect axes_authority -> Available
            }
            in_control {
                guard  axes_authority==Used
                effect axes_authority -> Available
            }
        }
        progress {
            period  1 sec
            message height: float
        }
        interrupt {
            interrupting true
            effect axes_authority -> Available
        }
        success at_altitude {
            effect {
                axes_authority -> Available
                flight_status  -> Hovering
            }
        }
        failure {
            blocked_on_ground {
                effect {
                    axes_authority -> Available
                    flight_status  -> OnGround
                }
            }
            blocked_in_air {
                effect {
                    axes_authority -> Available
                    flight_status  -> Hovering
                }
            }
        }
    }
*/
    skill goto {
        precondition {
            has_authority: authority != Pilot
            in_air       : flight_status != OnGround
        }
        start flight_status -> Moving
        invariant {
            keep_authority {
                guard authority == Drone
            }
            moving {
                guard flight_status == Moving
            }
        }
        success ok {
            effect flight_status -> Hovering
        }
        failure ko {}
    }

    skill goto_sol_1 {
        precondition {
            has_authority: authority == Drone
            in_air       : flight_status != OnGround
        }
        start flight_status -> Moving
        invariant {
            keep_authority {
                guard authority == Drone
            }
            moving {
                guard flight_status == Moving
            }
        }
        success ok {
            effect flight_status -> Hovering
        }
        failure ko {}
    }

    skill goto_sol_2 {
        precondition {
            has_authority: authority != Pilot
            in_air       : flight_status != OnGround
        }
        start {
            flight_status -> Moving
            authority -> Drone
        }
        invariant {
            keep_authority {
                guard authority == Drone
            }
            moving {
                guard flight_status == Moving
            }
        }
        success ok {
            effect flight_status -> Hovering
        }
        failure ko {}
    }}