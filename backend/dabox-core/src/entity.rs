/// This module provides an `Entity` trait that is used to identify entities in the system.
/// An entity has a unique identifier that is used to identify it in the system (i.e managing directory permissions).
pub type EntityUid = u64;

pub trait Entity {
    fn uid(&self) -> EntityUid;
}

pub struct StaticEntity(EntityUid);

impl Entity for StaticEntity {
    fn uid(&self) -> EntityUid {
        self.0
    }
}

impl StaticEntity {
    pub fn root() -> Self {
        Self(0)
    }
}
