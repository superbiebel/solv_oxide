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
            Ordering::Greater
        } else if self.hard_score < other.hard_score {
            Ordering::Less
        } else if self.hard_score == other.hard_score && self.soft_score > other.soft_score {
            Ordering::Greater
        } else if self.hard_score == other.hard_score && self.soft_score < other.soft_score {
            Ordering::Less
        } else if self.hard_score == other.hard_score && self.soft_score == other.soft_score {
            Ordering::Equal
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
#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::builtin::score::hard_soft_score::HardSoftScore;

    #[test]
    fn compare_test() {
        let test_data = vec!(((3,0),(2,0), Ordering::Greater),
                               ((2,0),(3,0), Ordering::Less),
                               ((3,10),(2,200), Ordering::Greater),
                               ((3,0),(3,0), Ordering::Equal),
                               ((3,1),(3,0), Ordering::Greater),
                               ((3,2),(3,4), Ordering::Less),
                               ((3,5),(2,5), Ordering::Greater));
        for data in test_data {
            let first = HardSoftScore {
                hard_score: data.0.0,
                soft_score: data.0.1,
            };
            let second = HardSoftScore {
                hard_score: data.1.0,
                soft_score: data.1.1,
            };
            assert_eq!(data.2, first.cmp(&second));
            assert_eq!(data.2, first.partial_cmp(&second).unwrap());
        }
    }
    #[test]
    fn equal_test() {
        let test_data = vec!(((3,0),(2,0), false),
                             ((2,0),(3,0), false),
                             ((3,10),(2,200), false),
                             ((3,0),(3,0), true),
                             ((3,1),(3,0), false),
                             ((3,2),(3,4), false),
                             ((3,5),(2,5), false));

        for data in test_data {
            let first = HardSoftScore {
                hard_score: data.0.0,
                soft_score: data.0.1,
            };
            let second = HardSoftScore {
                hard_score: data.1.0,
                soft_score: data.1.1,
            };
            assert_eq!(data.2, first.eq(&second));
        }
    }
    #[test]
    fn clone_test() {
        let first = HardSoftScore {
            hard_score: 100,
            soft_score: 200,
        };
        let second = first.clone();
        assert!(first == second);
    }
}