use std::marker::PhantomData;

use crate::interface::{MoveDecider, Phase, Score, StopType, Termination};

struct StandardPhase<SolutionType, ScoreType, MoveChangeGeneratorType, ScoreCalculatorType, MoveDeciderType, MoveGeneratorType>
    where MoveDeciderType: MoveDecider<SolutionType, ScoreType, MoveChangeGeneratorType> {
    move_decider: MoveDeciderType,
    move_generator: MoveGeneratorType,
    score_calculator: ScoreCalculatorType,
    phantom: PhantomData<SolutionType>,
    phantom2: PhantomData<ScoreType>,
    phantom3: PhantomData<MoveChangeGeneratorType>,
}

impl<'c, SolutionType, ScoreType, MoveChangeType, ScoreCalculatorType, TerminationType, MoveDeciderType, MoveGeneratorType> Phase<SolutionType, StandardPhaseScope<'c, SolutionType, ScoreType, TerminationType>> for StandardPhase<SolutionType, ScoreType, MoveChangeType, ScoreCalculatorType, MoveDeciderType, MoveGeneratorType>
where TerminationType: Termination<SolutionType, ScoreType>, MoveDeciderType: MoveDecider<SolutionType, ScoreType, MoveChangeType> {
    fn start_solving<'a>(&mut self, solution: &mut SolutionType, phase_scope: StandardPhaseScope<SolutionType, ScoreType, TerminationType>) {
        let termination = phase_scope.termination;
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
pub struct StandardPhaseScope<'b, SolutionType, ScoreType, TerminationType> where TerminationType: Termination<SolutionType, ScoreType> {
    pub termination: &'b mut TerminationType,
    pub phantom: PhantomData<SolutionType>,
    pub phantom2: PhantomData<ScoreType>
}