use std::marker::PhantomData;

use crate::interface::{ExecutableMove, MoveDecider, MoveGenerator, Phase, ScoreCalculator, StopType, Termination};

struct StandardPhase<SolutionType, ScoreType, MoveChangeType, ScoreCalculatorType, MoveDeciderType, MoveGeneratorType, MoveIteratorType>
    where MoveDeciderType: MoveDecider<SolutionType, ScoreType, MoveChangeType>,
          MoveIteratorType:Iterator<Item = Box<dyn ExecutableMove<SolutionType, MoveChangeType>>>,
          MoveGeneratorType: MoveGenerator<SolutionType, MoveChangeType, MoveIteratorType>,
          ScoreCalculatorType: ScoreCalculator<SolutionType, ScoreType, MoveChangeType> {
    move_decider: MoveDeciderType,
    move_generator: MoveGeneratorType,
    score_calculator: ScoreCalculatorType,
    phantom: PhantomData<SolutionType>,
    phantom2: PhantomData<ScoreType>,
    phantom3: PhantomData<MoveChangeType>,
    phantom4: PhantomData<MoveIteratorType>
}

impl<'c, SolutionType, ScoreType, MoveChangeType, ScoreCalculatorType, TerminationType, MoveDeciderType, MoveGeneratorType, MoveIteratorType> Phase<SolutionType, StandardPhaseScope<'c, SolutionType, ScoreType, TerminationType>>
for StandardPhase<SolutionType, ScoreType, MoveChangeType, ScoreCalculatorType, MoveDeciderType, MoveGeneratorType, MoveIteratorType>
where TerminationType: Termination<SolutionType, ScoreType>,
      MoveIteratorType:Iterator<Item = Box<dyn ExecutableMove<SolutionType, MoveChangeType>>>,
      MoveGeneratorType: MoveGenerator<SolutionType, MoveChangeType, MoveIteratorType>,
      MoveDeciderType: MoveDecider<SolutionType, ScoreType, MoveChangeType>,
      ScoreCalculatorType: ScoreCalculator<SolutionType, ScoreType, MoveChangeType>{
    fn start_solving<'a>(&mut self, solution: &mut SolutionType, phase_scope: StandardPhaseScope<SolutionType, ScoreType, TerminationType>) {
        let termination = phase_scope.termination;
        let last_score:Option<ScoreType> = None;
        #[cfg(feature = "builtin_standard_phase_move_decider_corrupt_check")]
        {
            self.move_decider.clear_current_run();
        }
        loop {
            let mut eval_moves = self.move_generator.generate(0, solution);
            let mut opt_move = None;
            loop {
                let eval_move_opt = eval_moves.next();
                let last = eval_move_opt.is_none();
                let eval_move = eval_move_opt.unwrap();
                if eval_move.is_doable(solution) {
                    continue;
                }
                let result = self.move_decider.should_apply(eval_move, self.score_calculator.calculate_score(solution), last);
                if let Some(step_result) = result {
                    opt_move = Some(step_result);
                    break;
                }
            };
            let result_move = opt_move.unwrap();
            result_move.do_move(solution).expect("Move did not return an undo move, this means the move wasn't doable even tho it said that it was when being evaluated.");
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