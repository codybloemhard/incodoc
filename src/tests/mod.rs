macro_rules! props {
    ($slice:expr) => {
        HashMap::from($slice)
    }
}

macro_rules! hset {
    ($slice:expr) => {
        HashSet::from_iter($slice.iter().map(|s| s.to_string()))
    }
}

pub mod parse;
pub mod squash;
pub mod prune;
pub mod toc;

