mod mutation;
mod query;

pub use mutation::{delete, dislike, like, publish, restore, set};
pub use query::{info, list, search, status};
