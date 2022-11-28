extern crate core;

use std::rc::Rc;
use unsafe_fn::unsafe_fn;
use solv_oxide::interface::{ProblemModel, SolutionModel, ScoreCalculator, IncrementalScoreCalculator, ExecutableMove};
use solv_oxide::builtin::score::hard_soft_score::HardSoftScore;

#[test]
fn knapsack_test() {
    struct KnapsackSolution<const N: usize>{
        items: [bool;N],
        problem: Rc<KnapsackProblem<N>>
    }
    struct KnapsackProblem<const N: usize> {
        amount: u16,
        cost: [u16;N],
    }

    impl Clone for KnapsackSolution<10> {
        fn clone(&self) -> Self {
            KnapsackSolution {
                items: self.items,
                problem: self.problem.clone(),
            }
        }
    }
    impl SolutionModel for KnapsackSolution<10> {}
    impl ProblemModel for KnapsackProblem<10> {}

    struct RandomFlipMove {
        item_nr: usize,
        value: bool
    }
    impl Clone for RandomFlipMove {
        fn clone(&self) -> Self {
            todo!()
        }
    }
    impl ExecutableMove<KnapsackSolution<10>, KnapsackMoveChange> for RandomFlipMove {
        #[unsafe_fn]
        fn do_move_unchecked(&self, solution: &mut KnapsackSolution<10>) -> (Box<dyn ExecutableMove<KnapsackSolution<10>, KnapsackMoveChange>>, KnapsackMoveChange) {
            solution.items[self.item_nr] = self.value;
            (Box::new(RandomFlipMove {
                item_nr: self.item_nr,
                value: !self.value,
            }), KnapsackMoveChange::None)
        }

        fn is_doable(&self, solution: &KnapsackSolution<10>) -> bool {
            solution.items[self.item_nr] == self.value
        }
    }
    enum KnapsackMoveChange{None}
    struct KnapsackScoreCalculator{}
    impl ScoreCalculator<KnapsackSolution<10>, HardSoftScore<i16, i16>, KnapsackMoveChange>
    for KnapsackScoreCalculator {
        fn calculate_score(&self, solution: &KnapsackSolution<10>) -> HardSoftScore<i16, i16> {
            let mut cost_amount = 0;
            for i in 0..solution.items.len()-1 {
                cost_amount += if solution.items[i] {
                    solution.problem.cost[i]
                } else {
                    0
                }
            }
            HardSoftScore {
                hard_score: 0,
                soft_score: -(cost_amount as i16 - 20).abs(),
            }
        }

        fn request_incremental(&self) -> Option<Box<dyn IncrementalScoreCalculator<KnapsackSolution<10>, HardSoftScore<i16, i16>, KnapsackMoveChange>>> {
            None
        }
    }
    let problem:KnapsackProblem<10> = KnapsackProblem {
        amount: 20,
        cost: [1,2,3,4,5,1,7,7,9,1],
    };
    //impl
    let model: KnapsackSolution<10> = KnapsackSolution {
        items: [false,false,false,false,false,false,false,false,false,false],
        problem: Rc::new(problem),
    };




    
}