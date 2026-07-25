#[allow(non_camel_case_types)]
pub enum State {
    FINISHED,
    IN_PROGRESS,
    IN_QUEUE,
    NOT_STARTED,
}

pub enum Relationship {
    REQUIRED,
    UNLOCKS,
    TEACHES,
}
