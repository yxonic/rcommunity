use rcommunity_core::{Enumerable, Numerical, Once, ReactionType};

#[derive(Clone, Copy)]
enum Vote {
    Upvote = 1,
    Downvote = -1,
}
impl ReactionType for Vote {}
impl Numerical for Vote {
    type Item = i32;
}
impl Enumerable for Vote {}
impl Once for Vote {}
