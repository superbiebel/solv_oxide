use std::time::{SystemTime, UNIX_EPOCH};
use crate::interface::Termination;

struct TimeTermination {
    max_time: u64,
    termination_level: TerminationLevel,
}
enum TerminationLevel{
    Solver {solver_start: Option<SystemTime>},
    Phase {phase_start: Option<SystemTime>},
    SolverAndPhase {
        solver_start:Option<SystemTime>,
        phase_start: Option<SystemTime>
    }
}

impl<SolutionType, ScoreType> Termination<SolutionType, ScoreType> for TimeTermination {
    fn solving_started(&mut self, solution: &SolutionType) -> bool {
        match self.termination_level {
            TerminationLevel::Solver { mut solver_start } => {
                solver_start = Option::from(SystemTime::now());
                true
            }
            TerminationLevel::Phase { mut phase_start } => {
                phase_start = None;
                true
            }
            TerminationLevel::SolverAndPhase { mut solver_start, mut phase_start } => {
                solver_start = Option::from(SystemTime::now());
                phase_start = None;
                true
            }
        }
    }

    fn phase_started(&mut self, solution: &SolutionType) -> bool {
        match self.termination_level {
            TerminationLevel::Solver { solver_start } => {
                match solver_start {
                    None => {
                        //If this happens it means the begin time of the solver wasn't initialized when it should. Check if you called solving_started before phase_started.
                        panic!("phase_started was called with a terminationLevel::solver and the begin time None")
                    }
                    Some(start) => {
                        if start.duration_since(UNIX_EPOCH).expect("time went backwards") - SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards") {

                        }
                    }
                }
                true
            }
            TerminationLevel::Phase { mut phase_start } => {
                phase_start = None;
                true
            }
            TerminationLevel::SolverAndPhase { mut solver_start, mut phase_start } => {
                solver_start = Option::from(SystemTime::now());
                phase_start = None;
                true
            }
        }
    }

    fn should_stop_solver(&self, score: &Option<ScoreType>, solution: &SolutionType) -> bool {
        todo!()
    }

    fn should_stop_phase(&self, score: &Option<ScoreType>, solution: &SolutionType) -> bool {
        todo!()
    }
}