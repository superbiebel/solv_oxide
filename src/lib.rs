#[cfg(feature = "builtin")]
#[allow(invalid_doc_attributes)]
pub mod builtin;
#[allow(invalid_doc_attributes)]
pub mod interface {
    use dyn_clonable::clonable;
    use unsafe_fn::unsafe_fn;

    pub(crate) trait Solver<SolutionType> {
        fn start_solving(&mut self, solution: &mut SolutionType);
    }

    pub(crate) trait Phase<SolutionType, PhaseScopeType> {
        fn start_solving(&mut self, solution: &mut SolutionType, phase_scope: PhaseScopeType);
    }
    pub trait Termination<SolutionType, ScoreType> {
        ///If this method returns false, stop immediately
        #[allow(unused_variables)]
        fn solver_started(&mut self, solution: &SolutionType) -> bool;

        #[allow(unused_variables)]
        fn solver_stopped(&mut self, solution: &SolutionType);

        ///If this method returns false, stop immediately
        #[allow(unused_variables)]
        fn phase_started(&mut self, solution: &SolutionType) -> bool;

        #[allow(unused_variables)]
        fn phase_stopped(&mut self, solution: &SolutionType);
        ///If this method returns false, stop immediately
        #[allow(unused_variables)]
        fn step_started(&mut self, solution: &SolutionType) -> bool;

        #[allow(unused_variables)]
        fn step_stopped(&mut self, solution: &SolutionType);
        ///If the Solver/Phase should stop immediately.
        #[allow(unused_variables)]
        fn should_stop(&self, score: &Option<ScoreType>, solution: &SolutionType) -> StopType {
            let stop_solver = self.should_stop_solver(score, solution);
            let stop_phase = self.should_stop_phase(score, solution);
            if stop_phase {
                StopType::StopPhase
            } else {
                StopType::StopSolver
            }
        }
        ///If this method returns false, stop immediately
        #[allow(unused_variables)]
        fn should_stop_solver(&self, score: &Option<ScoreType>, solution: &SolutionType) -> bool;
        #[allow(unused_variables)]
        fn should_stop_phase(&self, score: &Option<ScoreType>, solution: &SolutionType) -> bool;
    }
    #[derive(Debug, Eq, PartialEq)]
    pub enum StopType {
        None, StopSolver, StopPhase
    }

    ///This represents an algorithm like Hill Climbing which will choose which moves will be applied.
    pub(crate) trait MoveDecider<SolutionType, ScoreType, MoveChangeType> {
        fn should_apply(&mut self, move_check: Box<dyn ExecutableMove<SolutionType, MoveChangeType>>, score: ScoreType, islast: bool)
                        -> Option<Box<dyn ExecutableMove<SolutionType, MoveChangeType>>>;
        fn clear_current_run(&mut self);
        fn is_clear(&self) -> bool;
    }

    pub(crate) trait Score: Eq + Ord + Clone {}

    /// The model that has all the immutable variables that describe the problem.
    pub trait ProblemModel {}
    /// The model that has all the mutable variables that describe a possible solution that may or may not be invalid according to the calculated score
    pub trait SolutionModel : Clone{}
    pub enum NoneMoveChange{}

    ///This will decide how good a move is.
    pub trait ScoreCalculator<SolutionType, ScoreType, MoveChange> {
        ///This will always calculate the score from scratch not caching anything.
        fn calculate_score(&self, solution: &SolutionType) -> ScoreType;
        fn request_incremental(&self) -> Option<Box<dyn IncrementalScoreCalculator<SolutionType, ScoreType, MoveChange>>> {
            None
        }
    }

    pub trait IncrementalScoreCalculator<SolutionType, ScoreType, MoveChange> {
        fn clear(&mut self);
        fn variable_updated(&mut self);
        fn recalculate_score(&mut self, solution: &SolutionType) -> ScoreType;
    }

    pub trait MoveGenerator<SolutionType, MoveChangeType, MoveIteratorType: Iterator<Item = Box<dyn ExecutableMove<SolutionType, MoveChangeType>>>> {
        ///This method generates the moves that will be evaluated and scored.
        ///The `max_amount` arg will hint to the generator that giving more moves then this amount is useless and probably won't be evaluated.
        ///If you pass in 0 as max amount, it means that it is unlimited.
        fn generate(&mut self, max_amount: usize, solution: &SolutionType) -> MoveIteratorType;
    }
    #[clonable]
    pub trait ExecutableMove<SolutionType,MoveChangeType> : Clone {
        fn identifier(&self) -> String;
        fn do_move(&self, solution: &mut SolutionType) -> Option<(Box<dyn ExecutableMove<SolutionType, MoveChangeType>>, MoveChangeType)> {
            if self.is_doable(solution) {
                return Some(unsafe { self.do_move_unchecked(solution) });
            }
            None
        }

        /// This method should be implemented to do the actual change to the solution.
        /// It will NOT check if the move is actually doable on this solution. Panics may occur if the move is actually not doable.
        /// You need an unsafe block for this because it is unchecked and could put the entire thing in an inconsistent state.
        #[unsafe_fn]
        fn do_move_unchecked(&self, solution: &mut SolutionType) -> (Box<dyn ExecutableMove<SolutionType, MoveChangeType>>, MoveChangeType);
        fn is_doable(&self, solution: &SolutionType) -> bool;
    }
}

#[cfg(test)]
mod tests {
}
