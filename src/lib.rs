//! A Behavior Tree implementation for event logic and game AI
//!
//! Each action returns either `Success`, `Failure` or `Running`.
//! Actions are combined with behaviors such as `Wait` and `Select`.
//! The combined behavior is stored in a `Behavior` object.
//!
//! For each `Behavior` there is a `State`.
//! The state tracks the behavior over time.

#![crate_type = "lib"]
#![crate_name = "event"]
#![deny(missing_doc)]

extern crate piston;

pub use status::{
    Status,
    Success,
    Failure,
    Running,
};
pub use behavior::{
    Action,
    Behavior,
    Select,
    Sequence,
    Invert,
    Pressed,
    Released,
    Wait,
    WhenAll,
    While,
};
pub use state::{
    ActionState,
    PressedState,
    ReleasedState,
    SelectState,
    SequenceState,
    State,
    InvertState,
    WaitState,
    WhenAllState,
    WhileState,
};

mod state;
mod behavior;
mod status;
