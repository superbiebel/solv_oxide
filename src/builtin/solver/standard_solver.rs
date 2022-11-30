use std::marker::PhantomData;

use crate::builtin::phase::standard_phase::StandardPhaseScope;
use crate::builtin::termination::time_termination::{TerminationLevel, TimeTermination};
use crate::interface::{Phase, Solver, StopType, Termination};

struct StandardSolver<SolutionType, TerminationType, ScoreType> where TerminationType: Termination<SolutionType, ScoreType> {
    phases: Vec<Box<dyn Phase<SolutionType, StandardPhaseScope<SolutionType, ScoreType, TerminationType>>>>,
    termination: TerminationType,
    _phantom_score: PhantomData<ScoreType>,
}
impl<SolutionType, TerminationType, ScoreType> Solver<SolutionType> for StandardSolver<SolutionType, TerminationType, ScoreType>
    where TerminationType: Termination<SolutionType, ScoreType> {
    fn start_solving(&mut self, solution: &mut SolutionType) {

        for phase in &mut self.phases {

                if !TerminationType::phase_started(&mut self.termination, solution) {
                    return
                }


                phase.start_solving(solution, StandardPhaseScope {
                    termination: self.termination,
                    phantom: Default::default(),
                    phantom2: Default::default(),
                });

            match self.termination.should_stop(&None, solution) {
                StopType::StopSolver => {return}
                StopType::StopPhase => {}
                StopType::StopPhaseAndSolver => {return}
            }
        }
    }
}