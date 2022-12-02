use std::marker::PhantomData;

use crate::builtin::phase::standard_phase::StandardPhaseScope;
use crate::interface::{Phase, Solver, StopType, Termination};

#[allow(clippy::type_complexity)]
struct StandardSolver<SolutionType, TerminationType, ScoreType> where TerminationType: Termination<SolutionType, ScoreType> {
    phases: Vec<Box<dyn for <'a> Phase<SolutionType, StandardPhaseScope<'a, SolutionType, ScoreType, TerminationType>>>>,
    termination: TerminationType,
    _phantom_score: PhantomData<ScoreType>,
}
impl<SolutionType, TerminationType, ScoreType> Solver<SolutionType> for StandardSolver<SolutionType, TerminationType, ScoreType>
    where TerminationType: Termination<SolutionType, ScoreType> {
    fn start_solving(&mut self, solution: &mut SolutionType) {
        if !self.termination.solver_started(solution) {
            return;
        }

        for phase in &mut self.phases {
            if !self.termination.phase_started(solution) {
                return
            }
            phase.start_solving(solution, StandardPhaseScope {
                termination: &mut self.termination,
                phantom: Default::default(),
                phantom2: Default::default(),
            });

            match self.termination.should_stop(&None, solution) {
                StopType::StopSolver => { return }
                StopType::StopPhase => {}
                StopType::StopPhaseAndSolver => { return }
            }
        }
    }
    }