use std::mem;
use crate::interface::{ExecutableMove, MoveDecider, Score};

struct HillClimbingAlgorithm<SolutionType, ScoreType, MoveChangeType> {
    last_score: ScoreType,
    current_best_score: Option<ScoreType>,
    current_best_move: Option<Box<dyn ExecutableMove<SolutionType, MoveChangeType>>>
}
impl<SolutionType, ScoreType, MoveChangeType> MoveDecider<SolutionType, ScoreType, MoveChangeType> for HillClimbingAlgorithm<SolutionType, ScoreType, MoveChangeType>
where ScoreType: Score{
    fn should_apply(&mut self, move_check: Box<dyn ExecutableMove<SolutionType, MoveChangeType>>, score: ScoreType, islast: bool) -> Option<Box<dyn ExecutableMove<SolutionType, MoveChangeType>>> {
        score_move_none_corrupt_check(self);

        if self.last_score < score && self.current_best_score.is_some() && self.current_best_score.as_ref().unwrap() < &score {
            self.current_best_score = Some(score);
            self.current_best_move = Some(move_check)
        } else if self.current_best_score.is_none() {
            self.current_best_score = Some(score);
        }
        if islast && self.current_best_move.is_some() {
            let mut returnval = None;
            mem::swap(&mut returnval,&mut self.current_best_move);
            return returnval;
        }
        None
    }

    fn clear_current_run(&mut self) {
        self.current_best_score = None;
        self.current_best_move = None;
    }

    fn is_clear(&self) -> bool {

        self.current_best_score.is_none()
    }
}

fn score_move_none_corrupt_check<SolutionType, ScoreType, MoveChangeType>(to_check: &HillClimbingAlgorithm<SolutionType, ScoreType, MoveChangeType>) {
    #[cfg(feature = "builtin_hill_climbing_score_move_check")]
    {
        if to_check.current_best_score.is_none() == to_check.current_best_move.is_none() {
            panic!("Corrupted state! current score some is: {} while current best move some is: {}", &to_check.current_best_score.is_none().to_string(), &to_check.current_best_move.is_none().to_string());
        }
    }
}