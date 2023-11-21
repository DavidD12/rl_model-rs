type {
    FrameId
    Float
    Vector3
    Battery
    GeoPoint
}

skillset uav {
    data {
        battery: Battery
        position: GeoPoint
        home: GeoPoint
        // ground_distance: Float
    }

    resource {
        authority {
            // extern
            state { Pilot Skill }
            initial Pilot
            transition {
                Pilot -> Skill
                Skill -> Pilot
            }
        }

        home_status {
            // extern
            state { Invalid Valid }
            initial Invalid
            transition {
                Invalid -> Valid
                Valid -> Invalid
            }
        }

        flight_status {
            // extern
            state { NotReady OnGround InAir }
            initial NotReady
            transition all
        }

        motion {
            // internal
            state { Free Used }
            initial Free
            transition all
        }

        heading {
            // internal
            state { Free Used }
            initial Free
            transition all
        }

        battery {
            // extern
            state { Good Low Critical }
            initial Good
            transition {
                Good -> Low
                Good -> Critical
                Low  -> Critical
            }
        }
    }

    event {
        authority_to_pilot { 
            effect authority -> Pilot
        }
        authority_to_skill { 
            effect authority -> Skill
        }

        home_status_to_valid {
            effect home_status -> Valid
        }
        home_status_to_invalid {
            effect home_status -> Invalid
        }

        flight_status_to_not_ready { 
            effect flight_status -> NotReady
        }
        flight_status_to_on_ground { 
            effect flight_status -> OnGround
        }
        flight_status_to_in_air { 
            effect flight_status -> InAir
        }

        battery_to_low {
            guard battery == Good
            effect battery -> Low
        }
        battery_to_critical {
            effect battery -> Critical
        }
    }

    skill ask_authority {
        precondition {
            no_authority: authority == Pilot
        }
        interrupt {
            interrupting true
            postcondition authority == Pilot
        }
        success ok {
            postcondition authority == Skill
        }
        failure ko {
            postcondition authority == Pilot
        }
    }
    // property: not F(authority == Pilot and (motion == Used or heading == Used))
    // invariant: not (authority == Pilot and (motion == Used or heading == Used))

    skill capture_home {
        input {
            duration: Float // [s] Duration limit to catch gps signal and store the mean value
        }

        success ok {
            postcondition home_status == Valid
        }
        failure ko {
            postcondition home_status == Invalid
        }
    }

    skill takeoff {
        // TODO
        // parameter {
        //     height: Float // [m] validate can fail if h>h_geo_fence (valid range: [1.0; 7.0])
        //     speed:  Float // [m/s] maximum ascending velocity (valid range: ]0.0; 3.0]
        // }
        input {
            height: Float // [m] validate can fail if h>h_geo_fence (valid range: [1.0; 7.0])
        }

        precondition {
            has_authority: authority == Skill
            on_ground    : flight_status == OnGround
            not_moving   : motion == Free
            home_status  : home_status == Valid
            battery_good : battery == Good
        }
        start {
            motion -> Used
        }
        invariant {
            in_control {
                guard motion == Used
            }
            has_authority {
                guard  authority == Skill
                effect motion -> Free
            }
            battery {
                guard battery != Critical
                effect motion -> Free
            }
        }
        progress {
            period 1.0
            output height: Float
        }
        interrupt {
            interrupting true
            effect motion -> Free
        }
        success at_altitude {
            effect motion -> Free
            postcondition flight_status == InAir
        }
        failure {
            grounded {
                effect motion -> Free
                postcondition flight_status == OnGround
            }
            emergency {
                effect motion -> Free
                postcondition flight_status == InAir
            }
        }
    }

   skill goto {
        // TODO
        // parameter {
        //   validation_radius: Float // [m]   (>0, or -1 for automatic) a WP is considered as reached if distance(WP-current_position)<validation_radius and current_speed<validation_speed are both respected
        //   validation_speed:  Float // [m/s] (>0, or -1 for automatic) a WP is considered as reached if distance(WP-current_position)<validation_radius and current_speed<validation_speed are both respected
        //   wait_sec:          Float // >0: to wait <wait_sec> seconds before returning ARRIVED when arrived at point,
        //                              // =0: for no wait.
        //                              // Cannot be <0: use Hover skill instead after goto
        // }

        input {
            frame_id: FrameId // to choose behaviour of <target>: {ground_ENU, ground_NWU, ground_NED, ground_WGS84, global_WGS84, global_WGS84+und, body_FLU, body_FRD}
            target:   Vector3 // * for <frame_id>:={ground_WGS84, global_WGS84, global_WGS84+und}
                              //   =>  <target>:={x: latitude [deg], y: longitude [deg], z: altitude [m]}
                              // * for <frame_id>:={ground_ENU, ground_NWU, ground_NED, body_FLU, body_FRD}
                              //   =>    <target>:={x: x_position [m], y: y_poosition [m], z: z_position [m]}
            speed:             Float // [m/s] maximum translational speed (valid range: ]0.1; 15.0])
        }
        output distance : Float

        precondition {
            has_authority: authority == Skill
            in_air       : flight_status == InAir
            not_moving   : motion == Free
            home_status  : home_status == Valid
            battery_good : battery != Critical
        }
        start {
            motion -> Used
        }
        invariant {
            in_control {
                guard motion == Used
            }
            has_authority {
                guard  authority == Skill
                effect motion -> Free
            }
            in_air {
                guard flight_status == InAir
            }
            battery {
                guard battery != Critical
                effect motion -> Free
            }
        }
        progress {
            period 1.0
            output distance: Float
        }
        interrupt {
            interrupting true
            effect motion -> Free
        }
        success arrived {
            effect motion -> Free
        }
        failure emergency {
            // drifted, blocked, etc.
            effect motion -> Free
        }
    }

    skill landing {
        precondition {
            has_authority: authority == Skill
            in_air       : flight_status == InAir
            not_moving   : motion == Free
        }
        start {
            motion -> Used
        }
        invariant {
            in_control {
                guard motion == Used
            }
            has_authority {
                guard  authority == Skill
                effect motion -> Free
            }
        }
        progress {
            period 1.0
        }
        interrupt {
            interrupting true
            effect motion -> Free
        }
        success {
            on_ground {
                effect motion -> Free
                postcondition flight_status == OnGround
            }
            stopped {
                effect motion -> Free
                postcondition flight_status == NotReady
            }
        }
        failure {
            aborted {
                effect motion -> Free
                postcondition flight_status == InAir
            }
            emergency {
                // drifted, crashed, ...
                effect motion -> Free
            }
        }
    }

}