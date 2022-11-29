use std::marker::PhantomData;
use crate::interface::Phase;

struct StandardPhase<SolutionType, ScoreType, MoveChangeType, ScoreCalculatorType> {
    score_calculator: ScoreCalculatorType,
    phantom: PhantomData<SolutionType>,
    phantom2: PhantomData<ScoreType>,
    phantom3: PhantomData<MoveChangeType>,
}

impl<SolutionType, ScoreType, MoveChangeType, ScoreCalculatorType, PhaseScopeType> Phase<SolutionType, PhaseScopeType> for StandardPhase<SolutionType, ScoreType, MoveChangeType, ScoreCalculatorType> {
    fn start_solving(&mut self, solution: &mut SolutionType, phase_scope: PhaseScopeType) {

    }
}
pub struct StandardPhaseScope {
}