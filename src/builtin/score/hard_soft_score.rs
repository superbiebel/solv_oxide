use std::cmp::Ordering;
use crate::interface::Score;
pub struct HardSoftScore<HardScoreType, SoftScoreType> where HardScoreType: PartialEq + Ord + Clone, SoftScoreType: PartialEq + Ord + Clone{
    pub hard_score: HardScoreType,
    pub soft_score: SoftScoreType,
}

impl<HardScoreType, SoftScoreType> Eq for HardSoftScore<HardScoreType, SoftScoreType> where HardScoreType: PartialEq + Ord + Clone, SoftScoreType: PartialEq + Ord + Clone {}

impl<HardScoreType, SoftScoreType> PartialEq<Self> for HardSoftScore<HardScoreType, SoftScoreType> where HardScoreType: PartialEq + Ord + Clone, SoftScoreType: PartialEq + Ord + Clone {
    fn eq(&self, other: &Self) -> bool {
        self.hard_score == other.hard_score && self.soft_score == other.soft_score
    }
}

impl<HardScoreType, SoftScoreType> Ord for HardSoftScore<HardScoreType, SoftScoreType> where HardScoreType: PartialEq + Ord + Clone, SoftScoreType: PartialEq + Ord + Clone {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hard_score > other.hard_score {
            return Ordering::Greater;
        } else if self.hard_score < other.hard_score {
            return Ordering::Less;
        } else if self.hard_score == other.hard_score && self.soft_score > other.soft_score {
            Ordering::Greater
        } else if self.hard_score == other.hard_score && self.soft_score < other.soft_score {
            Ordering::Less
        } else if self.hard_score == other.hard_score && self.soft_score == other.soft_score {
            return Ordering::Equal
        } else {
            unreachable!("Could not order Hard/Soft score")
        }
    }
}

impl<HardScoreType, SoftScoreType> PartialOrd<Self> for HardSoftScore<HardScoreType, SoftScoreType> where HardScoreType: PartialEq + Ord + Clone, SoftScoreType: PartialEq + Ord + Clone {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<HardScoreType, SoftScoreType> Clone for HardSoftScore<HardScoreType, SoftScoreType> where HardScoreType: PartialEq + Ord + Clone, SoftScoreType: PartialEq + Ord + Clone {
    fn clone(&self) -> Self {
        HardSoftScore {
            hard_score: self.hard_score.clone(),
            soft_score: self.soft_score.clone(),
        }
    }
}

impl<HardScoreType, SoftScoreType> Score for HardSoftScore<HardScoreType, SoftScoreType>
    where HardScoreType: Ord + PartialEq + Clone, SoftScoreType: Ord + PartialEq + Clone {}



/*impl PartialEq for ScoreImpl<i64, i64> {
    fn eq(&self, other: &Self) -> bool {
        self.hard_score == other.hard_score &&
            self.soft_score == other.soft_score
    }
}

impl PartialOrd for ScoreImpl<i64, i64> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hard_score > other.hard_score {
            Some(Ordering::Greater)
        } else if self.soft_score > other.hard_score{
            Some(O)
        }
    }
}

impl Clone for ScoreImpl<i64, i64> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Score for ScoreImpl<i64, i64> {

}*/