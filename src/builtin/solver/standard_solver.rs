use crate::interface::{Phase, Solver, Termination};

struct StandardSolver<SolutionType, PhaseScopeType, TerminationType, ScoreType> where TerminationType: Termination<SolutionType, ScoreType> {
    phases: Vec<Box<dyn Phase<SolutionType, PhaseScopeType>>>,
    termination: TerminationType,
    phantom_score: ScoreType
}
impl<SolutionType, PhaseScopeType, TerminationType, ScoreType> Solver<SolutionType> for StandardSolver<SolutionType, PhaseScopeType, TerminationType, ScoreType> where TerminationType: Termination<SolutionType, ScoreType> {
    fn start_solving(&mut self, solution: &mut SolutionType) {
        if !self.termination.solver_started(solution) {
            return;
        }
        for phase in &mut self.phases {
            if !self.termination.phase_started(solution) {
                todo!("Standard Solver"); //TODO implement phase solving for StandardSolver
                return;
            }
        }

    }
}