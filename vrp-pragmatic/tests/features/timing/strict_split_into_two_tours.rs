use crate::format::problem::*;
use crate::format::solution::*;
use crate::helpers::*;

#[test]
fn can_split_into_two_tours_because_of_strict_times() {
    let problem = Problem {
        plan: Plan {
            jobs: vec![
                create_delivery_job_with_times("job1", vec![10., 0.], vec![(70, 80)], 10.),
                create_delivery_job_with_times("job2", vec![20., 0.], vec![(50, 60)], 10.),
                create_delivery_job_with_times("job3", vec![30., 0.], vec![(0, 40), (100, 120)], 10.),
                create_delivery_job_with_times("job4", vec![40., 0.], vec![(0, 40)], 10.),
                create_delivery_job_with_times("job5", vec![50., 0.], vec![(50, 60)], 10.),
            ],
            relations: Option::None,
        },
        fleet: Fleet {
            vehicles: vec![VehicleType {
                vehicle_ids: vec!["my_vehicle_1".to_string(), "my_vehicle_2".to_string()],
                ..create_default_vehicle_type()
            }],
            profiles: create_default_matrix_profiles(),
        },
        objectives: create_min_jobs_cost_objective(),
        ..create_empty_problem()
    };
    let matrix = create_matrix_from_problem(&problem);

    let solution = solve_with_metaheuristic(problem, Some(vec![matrix]));

    assert_vehicle_agnostic(
        solution,
        Solution {
            statistic: Statistic {
                cost: 360.,
                distance: 140,
                duration: 200,
                times: Timing { driving: 140, serving: 50, waiting: 10, break_time: 0 },
            },
            tours: vec![
                Tour {
                    vehicle_id: "my_vehicle_1".to_string(),
                    type_id: "my_vehicle".to_string(),
                    shift_index: 0,
                    stops: vec![
                        create_stop_with_activity(
                            "departure",
                            "departure",
                            (0., 0.),
                            3,
                            ("1970-01-01T00:00:00Z", "1970-01-01T00:00:00Z"),
                            0,
                        ),
                        create_stop_with_activity(
                            "job4",
                            "delivery",
                            (40., 0.),
                            2,
                            ("1970-01-01T00:00:40Z", "1970-01-01T00:00:50Z"),
                            40,
                        ),
                        create_stop_with_activity(
                            "job5",
                            "delivery",
                            (50., 0.),
                            1,
                            ("1970-01-01T00:01:00Z", "1970-01-01T00:01:10Z"),
                            50,
                        ),
                        create_stop_with_activity(
                            "job3",
                            "delivery",
                            (30., 0.),
                            0,
                            ("1970-01-01T00:01:30Z", "1970-01-01T00:01:50Z"),
                            70,
                        ),
                        create_stop_with_activity(
                            "arrival",
                            "arrival",
                            (0., 0.),
                            0,
                            ("1970-01-01T00:02:20Z", "1970-01-01T00:02:20Z"),
                            100,
                        ),
                    ],
                    statistic: Statistic {
                        cost: 250.,
                        distance: 100,
                        duration: 140,
                        times: Timing { driving: 100, serving: 30, waiting: 10, break_time: 0 },
                    },
                },
                Tour {
                    vehicle_id: "my_vehicle_2".to_string(),
                    type_id: "my_vehicle".to_string(),
                    shift_index: 0,
                    stops: vec![
                        create_stop_with_activity(
                            "departure",
                            "departure",
                            (0., 0.),
                            2,
                            ("1970-01-01T00:00:00Z", "1970-01-01T00:00:30Z"),
                            0,
                        ),
                        create_stop_with_activity(
                            "job2",
                            "delivery",
                            (20., 0.),
                            1,
                            ("1970-01-01T00:00:50Z", "1970-01-01T00:01:00Z"),
                            20,
                        ),
                        create_stop_with_activity(
                            "job1",
                            "delivery",
                            (10., 0.),
                            0,
                            ("1970-01-01T00:01:10Z", "1970-01-01T00:01:20Z"),
                            30,
                        ),
                        create_stop_with_activity(
                            "arrival",
                            "arrival",
                            (0., 0.),
                            0,
                            ("1970-01-01T00:01:30Z", "1970-01-01T00:01:30Z"),
                            40,
                        ),
                    ],
                    statistic: Statistic {
                        cost: 110.,
                        distance: 40,
                        duration: 60,
                        times: Timing { driving: 40, serving: 20, waiting: 0, break_time: 0 },
                    },
                },
            ],
            ..create_empty_solution()
        },
    );
}
