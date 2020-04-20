use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Transmutations require materials and produce a product.
pub trait Transmutation {
    fn materials(&self) -> &[TypeId];
    fn product(&self) -> TypeId;
    fn transmute(&self, inputs: &[&dyn Any]) -> Box<dyn Any>;
}

pub struct Tome {
    /// Transmutations observed to happen naturally are transcribed here.
    /// The transmutations are organized by their products to discover new recepies.
    notes: HashMap<TypeId, Box<dyn Transmutation>>,
    /// Circles are inscribed here so that they be transcribed later.
    circles: Vec<Box<dyn Transmutation>>,
}

impl Tome {
    /// Inscribe a note about a natural transmutation into the tome.
    pub fn note<T: Transmutation + 'static>(&mut self, note: T) {
        self.notes.insert(note.product(), Box::new(note));
    }

    /// Inscribe a circle into the tome and give back the page of the inscription.
    pub fn circle<T: Transmutation + 'static>(&mut self, circle: T) -> usize {
        let page = self.circles.len();
        self.circles.push(Box::new(circle));
        page
    }

    /// Give me what I want.
    pub fn summon<T>(&self) -> T {
        self.preserve(&mut Materials::new())
    }

    /// Give me what I want, but preserve the materials created in the transmutation.
    pub fn preserve<T>(&self, materials: &mut Materials) -> T {
        unimplemented!()
    }
}

/// The materials we have previously produced.
#[derive(Default)]
pub struct Materials {
    materials: HashMap<TypeId, Box<dyn Any>>,
}

impl Materials {
    fn new() -> Self {
        Self::default()
    }
}
