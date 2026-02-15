use crate::*;

/// Merge two objects by having one absorb the other.
pub trait Absorb {
    type Other;
    /// Absorb other into self.
    fn absorb(&mut self, other: Self::Other);
}

impl Absorb for Tags {
    type Other = Option<Self>;
    fn absorb(&mut self, other: Self::Other) {
        if let Some(o) = other {
            for v in o {
                self.insert(v);
            }
        }
    }
}

impl Absorb for Props {
    type Other = Self;
    fn absorb(&mut self, other: Self::Other) {
        for prop in other {
            insert_prop(self, prop);
        }
    }
}

