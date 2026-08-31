use crate::enumerations::{Relationship, State};

impl TryFrom<i32> for State {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(State::FINISHED),
            1 => Ok(State::IN_PROGRESS),
            2 => Ok(State::IN_QUEUE),
            3 => Ok(State::NOT_STARTED),
            _ => Err(()), // can't panic here
        }
    }
}

impl TryFrom<i32> for Relationship {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Relationship::REQUIRED),
            1 => Ok(Relationship::UNLOCKS),
            2 => Ok(Relationship::TEACHES),
            _ => Err(()),
        }
    }
}
