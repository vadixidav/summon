use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Transmutations require ingredients and produce a product.
pub trait Transmutation {
    fn ingredients(&self) -> &[TypeId];
    fn product(&self) -> TypeId;
    fn transmute(&self, inputs: &[&dyn Any]) -> Box<dyn Any>;
}

/// This is where all of the transmutation circles are inscribed.
pub struct Tome {
    /// Transmutation circles are organized by their products in the tomb.
    circles: HashMap<TypeId, Vec<Box<dyn Transmutation>>>,
}

impl Tome {
    /// Inscribe a note about a natural transmutation into the tome.
    pub fn inscribe<T: Transmutation + 'static>(&mut self, note: T) {
        self.circles
            .entry(note.product())
            .or_default()
            .push(Box::new(note));
    }

    /// Give me what I want.
    pub fn summon<T>(&self) -> Option<T> {
        unimplemented!()
    }

    /// Give me what I want, but inscribe the process so it can be more quickly executed in the future.
    pub fn preserve<T>(&mut self) -> Option<T> {
        unimplemented!()
    }
}
