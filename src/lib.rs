trait Solver<SolutionType> {
    fn start_solving(&self, solution: &mut SolutionType);
}
trait Phase<SolutionType>{
    fn start_solving(solution: &mut SolutionType);
}
trait MoveDecider<SolutionType>{
    fn should_apply(&mut self, move_check: &dyn ExecutableMove<SolutionType>);
}
trait Score{}
/// The model that has all the immutable variables that describe the problem.
trait ProblemModel{}
trait SolutionModel{}
trait ScoreCalculator<SolutionType, ScoreType> {
    fn calculate_score(solution: &SolutionType) -> ScoreType;
}
trait ExecutableMove<SolutionType> {
    fn do_move(&self, solution: &mut SolutionType) -> Option<Box<dyn ExecutableMove<SolutionType>>> {
        if self.is_doable(solution) {
            return Some(self.do_move_unchecked(solution));
        }
        None
    }
    /// this method should be implemented to do the actual change to the solution. 
    /// It will NOT check if the move is actually doable on this solution. Panics may occur if the move is actually not doable.
    fn do_move_unchecked(&self, solution: &mut SolutionType) -> Box<dyn ExecutableMove<SolutionType>>;
    fn is_doable(&self, solution: &SolutionType) -> bool;
}

#[cfg(test)]
mod tests {

}
