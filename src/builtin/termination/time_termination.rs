use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::interface::{Termination};

struct TimeTermination {
    termination_level: TerminationLevel,
}
enum TerminationLevel{
    Solver {
        solver_start: Option<SystemTime>,
        max_time: Duration,
    },
    Phase {
        phase_start: Option<SystemTime>,
        max_time: Duration,
    }
}

impl<SolutionType, ScoreType> Termination<SolutionType, ScoreType> for TimeTermination {
    fn solver_started(&mut self, _solution: &SolutionType) -> bool {
        match self.termination_level {
            TerminationLevel::Solver {max_time,..} => {
                self.termination_level = TerminationLevel::Solver {
                    solver_start: Some(SystemTime::now()),
                    max_time,
                };
                true
            }
            _ => {
                true
            }
        }
    }

    fn phase_started(&mut self, _solution: &SolutionType) -> bool {
        match self.termination_level {
            TerminationLevel::Solver { solver_start, max_time } => {
                //If this happens it means the begin time of the solver wasn't initialized when it should. Check if you called solving_started before phase_started.
                !check_if_longer(solver_start.expect("phase_started was called with a terminationLevel::Solver and the solver begin time None"), &max_time)
            }
            TerminationLevel::Phase { mut phase_start, .. } => {
                phase_start = Some(SystemTime::now());
                self.termination_level = TerminationLevel::Phase {
                    phase_start,
                    max_time: Default::default(),
                };
                true
            }
        }
    }

    fn should_stop_solver(&self, _score: &Option<ScoreType>, _solution: &SolutionType) -> bool {
        match self.termination_level {
            TerminationLevel::Solver {solver_start, max_time } => {
                check_if_longer(solver_start.expect("State corrupted: tried to check Solver time while it was not initialized"),&max_time)
            }
            TerminationLevel::Phase { .. } => {false}
        }
    }

    fn should_stop_phase(&self, _score: &Option<ScoreType>, _solution: &SolutionType) -> bool {
        match self.termination_level {
            TerminationLevel::Solver { .. } => {false}
            TerminationLevel::Phase { phase_start, max_time } => {
                let phase = phase_start.expect("State corrupted: tried to check Phase time while it was not initialized");
                check_if_longer(phase, &max_time)
            }
        }
    }
}

fn check_if_longer(start: SystemTime, duration: &Duration) -> bool {
     SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards") - start.duration_since(UNIX_EPOCH).expect("time went backwards") >= *duration
}
#[cfg(test)]
mod tests {
    use std::thread::{sleep};
    use std::time::{Duration};
    use crate::builtin::termination::time_termination::{TerminationLevel, TimeTermination};
    use crate::interface::StopType::{StopPhase, StopSolver};
    use crate::interface::Termination;

    struct TestSolution{}
    struct TestScore{}

    #[test]
    fn time_test_phase() {
        let solution = TestSolution{};

        let mut termination = TimeTermination {
            termination_level: TerminationLevel::Phase {
                phase_start: None,
                max_time: Duration::from_secs(1),
            },
        };

        <TimeTermination as Termination<TestSolution, TestScore>>::solver_started(&mut termination, &solution);
        <TimeTermination as Termination<TestSolution, TestScore>>::phase_started(&mut termination, &solution);
        sleep(Duration::from_secs(2));
        assert_eq!(StopPhase,<TimeTermination as Termination<TestSolution, TestScore>>::should_stop(&termination, &None, &solution));
    }
    #[test]
    fn time_test_solver() {
        let solution = TestSolution{};

        let mut termination = TimeTermination {
            termination_level: TerminationLevel::Solver {
                solver_start: None,
                max_time: Duration::from_secs(1),
            },
        };

        <TimeTermination as Termination<TestSolution, TestScore>>::solver_started(&mut termination, &solution);
        <TimeTermination as Termination<TestSolution, TestScore>>::phase_started(&mut termination, &solution);
        sleep(Duration::from_secs(2));
        assert_eq!(StopSolver,<TimeTermination as Termination<TestSolution, TestScore>>::should_stop(&termination, &None, &solution));
    }
}