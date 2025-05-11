#[derive(Debug, PartialEq, Eq)]
pub enum RollResultType {
    Normal,
    Great,
    Extreme,
    Critical,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RollResult {
    Success(RollResultType),
    Failure(RollResultType),
}

pub trait Rollable {
    fn check(&self, roll: i32) -> RollResult;
    fn roll(&self) -> RollResult;
}
