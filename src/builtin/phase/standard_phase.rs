use std::marker::PhantomData;

use crate::interface::{Phase, Score, StopType, Termination};

struct StandardPhase<SolutionType, ScoreType, MoveChangeType, ScoreCalculatorType> {
    score_calculator: ScoreCalculatorType,
    phantom: PhantomData<SolutionType>,
    phantom2: PhantomData<ScoreType>,
    phantom3: PhantomData<MoveChangeType>,
}

impl<SolutionType, ScoreType, MoveChangeType, ScoreCalculatorType, TerminationType> Phase<SolutionType, StandardPhaseScope<SolutionType, ScoreType, TerminationType>> for StandardPhase<SolutionType, ScoreType, MoveChangeType, ScoreCalculatorType>
where TerminationType: Termination<SolutionType, ScoreType> {
    fn start_solving<'a>(&mut self, solution: &mut SolutionType, phase_scope: StandardPhaseScope<SolutionType, ScoreType, TerminationType>) {
        let termination = &phase_scope.termination;
        let last_score:Option<ScoreType> = None;
        loop {
            match termination.should_stop(&last_score, solution) {
                StopType::StopSolver => {}
                StopType::StopPhase => {break}
                StopType::StopPhaseAndSolver => {break}
            }
        }
    }
}
pub struct StandardPhaseScope<SolutionType, ScoreType, TerminationType> where TerminationType: Termination<SolutionType, ScoreType> {
    pub(crate) termination: TerminationType,
    pub(crate) phantom: PhantomData<SolutionType>,
    pub(crate) phantom2: PhantomData<ScoreType>
}