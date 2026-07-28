/// Enum used to define the state of the Node
/// 
/// Always defaults to NOT_STARTED
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Default)]
pub enum State{
    FINISHED,
    IN_PROGRESS,
    IN_QUEUE,
    #[default] NOT_STARTED
}

/// Enum used to define the type of the relationship
/// 
/// Does not have a default value
#[derive(Debug, Clone, Copy)]
pub enum Relationship{
    REQUIRED,
    UNLOCKS,
    TEACHES
}