use std::mem;

use crate::interface::{ExecutableMove, MoveDecider, Score};

struct HillClimbingAlgorithm<SolutionType, ScoreType, MoveChangeType> {
    last_score: ScoreType,
    current_best_score: Option<ScoreType>,
    current_best_move: Option<Box<dyn ExecutableMove<SolutionType, MoveChangeType>>>
}
impl<SolutionType, ScoreType, MoveChangeType> MoveDecider<SolutionType, ScoreType, MoveChangeType> for HillClimbingAlgorithm<SolutionType, ScoreType, MoveChangeType>
where ScoreType: Score{
    #[allow(clippy::if_same_then_else)]
    fn should_apply(&mut self, move_check: Box<dyn ExecutableMove<SolutionType, MoveChangeType>>, score: ScoreType, islast: bool) -> Option<Box<dyn ExecutableMove<SolutionType, MoveChangeType>>> {
        score_move_none_corrupt_check(self);
        if self.last_score < score && self.current_best_score.is_some() && self.current_best_score.as_ref().unwrap() < &score {
            self.current_best_score = Some(score);
            self.current_best_move = Some(move_check)
        } else if self.current_best_score.is_none() {
            self.current_best_score = Some(score);
            self.current_best_move = Some(move_check)
        }
        if islast && self.current_best_move.is_some() {
            let mut return_val = None;
            mem::swap(&mut return_val, &mut self.current_best_move);
            return return_val;
        }
        None
    }

    fn clear_current_run(&mut self) {
        self.current_best_score = None;
        self.current_best_move = None;
    }

    fn is_clear(&self) -> bool {
        self.current_best_score.is_none() && self.current_best_move.is_none()
    }
}

fn score_move_none_corrupt_check<SolutionType, ScoreType, MoveChangeType>(to_check: &HillClimbingAlgorithm<SolutionType, ScoreType, MoveChangeType>) {
    #[cfg(feature = "builtin_hill_climbing_score_move_check")]
    {
        if to_check.current_best_score.is_none() != to_check.current_best_move.is_none() {
            panic!("Corrupted state! current score some is: {} while current best move some is: {}", &to_check.current_best_score.is_none().to_string(), &to_check.current_best_move.is_none().to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use unsafe_fn::unsafe_fn;

    use crate::builtin::movedecider::hill_climbing::HillClimbingAlgorithm;
    use crate::builtin::score::hard_soft_score::HardSoftScore;
    use crate::interface::{ExecutableMove, MoveDecider};

    struct TestMove;
    const TEST_MOVE_IDENT: &str = "testmove";
    struct TestSolution;

    enum TestMoveChange {
        Test
    }
    impl Clone for TestMove {
        fn clone(&self) -> Self {
            TestMove{}
        }
    }
    impl ExecutableMove<TestSolution, TestMoveChange> for TestMove {
        fn identifier(&self) -> String {
            TEST_MOVE_IDENT.to_string()
        }

        #[unsafe_fn]
        fn do_move_unchecked(&self, _solution: &mut TestSolution) -> (Box<dyn ExecutableMove<TestSolution, TestMoveChange>>, TestMoveChange) {
            (Box::new(TestMove{}), TestMoveChange::Test)
        }

        fn is_doable(&self, _solution: &TestSolution) -> bool {
            true
        }
    }


    #[test]
    fn hill_climbing_accept_test() {
        let mut decider = HillClimbingAlgorithm::<TestSolution, HardSoftScore<i32, i32>, TestMoveChange> {
            last_score: HardSoftScore { hard_score: 0, soft_score: 0 },
            current_best_score: None,
            current_best_move: None,
        };
        let result1 =
            decider.should_apply(Box::new(TestMove{}), HardSoftScore { hard_score: 1, soft_score: 2 }, false);
        assert!(result1.is_none());
        let final_result =
            decider.should_apply(Box::new(TestMove{}), HardSoftScore { hard_score: 3, soft_score: 2 }, true);
        assert!(final_result.is_some());
    }
    #[test]
    fn hill_climbing_reject_test() {
        let mut decider = HillClimbingAlgorithm::<TestSolution, HardSoftScore<i32, i32>, TestMoveChange> {
            last_score: HardSoftScore { hard_score: 0, soft_score: 0 },
            current_best_score: None,
            current_best_move: None,
        };
        let result1 =
            decider.should_apply(Box::new(TestMove{}), HardSoftScore { hard_score: 1, soft_score: 2 }, false);
        assert!(result1.is_none());
        let final_result =
            decider.should_apply(Box::new(TestMove{}), HardSoftScore { hard_score: 3, soft_score: 2 }, true);
        assert!(final_result.is_some());
        assert_eq!(final_result.unwrap().identifier(), TEST_MOVE_IDENT.to_string())
    }
}