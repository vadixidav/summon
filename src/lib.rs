//! # summon
//!
//! A logic engine designed to magically give you what you ask for
//!
//! ```
//! #[derive(Clone)]
//! struct ConstantAcceleration(f64);
//! #[derive(Clone)]
//! struct InitialVelocity(f64);
//! #[derive(Clone)]
//! struct InitialPosition(f64);
//! #[derive(Clone)]
//! struct Time(f64);
//!
//! struct Distance(f64);
//! struct RealPhysicsOn;
//!
//! #[test]
//! fn sum() {
//!     // The tome is where all the logic and conversions are written in your code.
//!     let mut tome = Tome::new();
//!
//!     // You can use ether() to give types as givens.
//!     tome.ether(ConstantAcceleration(3.0));
//!     tome.ether(InitialVelocity(5.0));
//!     tome.ether(InitialPosition(6.0));
//!     tome.ether(Time(4.0));
//!     tome.ether(RealPhysicsOn);
//!
//!     // Inscribe is used to describe a conversion between types.
//!     // If a function produces multiple types via tuple, two extra inscriptions can be made to break it up into two types.
//!     tome.inscribe(
//!         circle!((_: &RealPhysicsOn, a: &ConstantAcceleration, v: &InitialVelocity, p: &InitialPosition, t: &Time) -> Distance {
//!             Distance(0.5 * a.0 * t.0.powi(2) + v.0 * t.0 + p.0)
//!         }),
//!     );
//!
//!     // So long as it is possible to produce the result with the given inscriptions, it will be produced.
//!     let summoned = tome.summon::<Distance>().unwrap();
//!     assert_eq!(
//!         0.5 * 3.0 * 4.0f64.powi(2) + 5.0 * 4.0 + 6.0,
//!         summoned,
//!     );
//! }
//! ```
//!

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::iter::FromIterator;

/// Transmutations require ingredients and produce a product. This is usually a function.
pub trait Transmutation {
    fn ingredients(&self) -> &'static [TypeId];
    fn product(&self) -> TypeId;
    fn transmute(&self, inputs: &[&dyn Any]) -> Box<dyn Any>;
}

struct Ether<T>(T);

impl<T: Clone + 'static> Transmutation for Ether<T> {
    fn ingredients(&self) -> &'static [TypeId] {
        &[]
    }
    fn product(&self) -> TypeId {
        TypeId::of::<T>()
    }
    fn transmute(&self, _: &[&dyn Any]) -> Box<dyn Any> {
        Box::new(self.0.clone())
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! transmutation_impl {
    (($($arg_real_pat:pat in $arg_ty:tt),*) -> $return_ty:tt $body:tt) => {{
        paste::expr! {{
            use std::any::{Any, TypeId};
            struct Temporary<F>(F);
            const TEMPORARY_INGREDIENTS: &[TypeId] = &[$(TypeId::of::<$arg_ty>()),*];
            impl<F: Fn($(&$arg_ty),*) -> $return_ty> $crate::Transmutation for Temporary<F> {
                fn ingredients(&self) -> &'static [TypeId] {
                    TEMPORARY_INGREDIENTS
                }
                fn product(&self) -> TypeId {
                    TypeId::of::<$return_ty>()
                }
                fn transmute(&self, inputs: &[&dyn Any]) -> Box<dyn Any> {
                    if let [$([<temp_ident_ $arg_ty>]),*] = inputs {
                        $(let [<temp_ident_ $arg_ty>] = [<temp_ident_ $arg_ty>].downcast_ref::<$arg_ty>().expect("transmute passed an incorrect type");)*
                        Box::new((self.0)($([<temp_ident_ $arg_ty>]),*)) as Box<dyn Any>
                    } else {
                        panic!("transmute passed incorrect number of arguments (expected: {}, found: {})", self.ingredients().len(), inputs.len());
                    }
                }
            }
            Temporary(|$($arg_real_pat: &$arg_ty),*| -> $return_ty $body)
        }}
    }};
}

/// Use this to inscribe a transmutation between a set of input types and an output type.
///
/// ```
/// # #![feature(const_type_id)]
/// use summon::{Tome, circle};
/// #[derive(Clone)]
/// struct Normal(u32);
/// struct Double(u32);
/// struct Half(u32);
/// let mut tome = Tome::new();
/// tome.ether(Normal(4));
/// tome.inscribe(summon::circle!(|n: &Normal| -> Double { Double(n.0 * 2) }));
/// tome.inscribe(summon::circle!(|n: &Normal| -> Half { Half(n.0 / 2) }));
/// assert_eq!(8, tome.summon::<Double>().unwrap().0);
/// assert_eq!(2, tome.summon::<Half>().unwrap().0);
/// ```
#[macro_export]
macro_rules! circle {
    (|$($arg_name:tt: &$arg_ty:ty),*| -> $return_ty:tt $body:tt) => {{
        $crate::transmutation_impl!(($($arg_name in $arg_ty),*) -> $return_ty $body)
    }};
}

/// Use this to inscribe tag type/zero-sized struct (`struct A;`) conversions. Useful for logic.
///
/// ```
/// # #![feature(const_type_id)]
/// use summon::{Tome, fusion};
/// #[derive(Clone)]
/// struct A;
/// struct B;
/// let mut tome = Tome::new();
/// tome.ether(A);
/// tome.inscribe(fusion!(A => B));
/// tome.summon::<B>().unwrap();
/// ```
#[macro_export]
macro_rules! fusion {
    ($($arg_name:ty),* => $return_ty:tt) => {
        $crate::transmutation_impl!(($(_ in $arg_name),*) -> $return_ty { $return_ty })
    };
}

#[macro_export]
macro_rules! bend {
    (($($arg_name:tt $arg_pat:tt),*) -> $return_ty:tt $return_pat:tt) => {{
        $crate::transmutation_impl!(($($arg_name $arg_pat in $arg_name),*) -> $return_ty { $return_ty $return_pat })
    }};
}

/// This is where all of the transmutation circles are inscribed.
#[derive(Default)]
pub struct Tome {
    /// Transmutation circles are organized by their products in the tomb.
    circles: HashMap<TypeId, Vec<Box<dyn Transmutation>>>,
}

impl Tome {
    /// Create an empty tome.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inscribe a note about a natural transmutation into the tome.
    pub fn inscribe<T: Transmutation + 'static>(&mut self, circle: T) {
        let product_circles = self.circles.entry(circle.product()).or_default();
        product_circles.push(Box::new(circle));
        product_circles.sort_by_key(|c| c.ingredients().len());
    }

    /// Create a note about how to create something out of the ether.
    pub fn ether<T: Clone + 'static>(&mut self, item: T) {
        self.inscribe(Ether(item));
    }

    /// Give me what I want.
    pub fn summon<T: 'static>(&self) -> Option<T> {
        // Preserve all the materials we need and the thing we are summoning.
        let materials = self.preserve::<T>()?;
        // Drop all the intermediate materials to get only the desired one.
        Some(materials.into_material::<T>())
    }

    /// Give me what I want and more.
    fn preserve<T: 'static>(&self) -> Option<Materials> {
        // Find a recipe to create the item. This may fail.
        let recipe: Recipe = self.research::<T>()?;
        // Perform the whole recipe. This cannot fail, excpet via panic.
        let materials: Materials = recipe.steps.into_iter().collect();
        // Create all the materials in the recipe.
        Some(materials)
    }

    fn research<T: 'static>(&self) -> Option<Recipe<'_>> {
        self.research_id(TypeId::of::<T>())
    }

    fn research_id(&self, id: TypeId) -> Option<Recipe<'_>> {
        self.circles.get(&id).and_then(|possibilities| {
            possibilities.iter().find_map(|circle| {
                let ingredients = circle.ingredients();
                eprintln!("ingredients: {}", ingredients.len());
                ingredients
                    .iter()
                    .fold(Some(Recipe::default()), |recipe, &ingredient| {
                        recipe.and_then(|recipe| {
                            self.research_id(ingredient).map(|next| recipe.join(next))
                        })
                    })
                    .map(|recipe| recipe.join((**circle).into()))
            })
        })
    }
}

#[derive(Default)]
struct Recipe<'a> {
    steps: Vec<&'a dyn Transmutation>,
    products: HashMap<TypeId, usize>,
}

impl<'a> From<&'a dyn Transmutation> for Recipe<'a> {
    fn from(circle: &'a dyn Transmutation) -> Self {
        let mut recipe = Self::default();
        recipe.products.insert(circle.product(), 0);
        recipe.steps.push(circle);
        recipe
    }
}

impl<'a> Recipe<'a> {
    fn join(self, other: Self) -> Self {
        let Self {
            mut steps,
            mut products,
        } = self;
        let Self {
            steps: other_steps,
            products: other_products,
        } = other;
        for (product, step) in other_products {
            products.entry(product).or_insert_with(|| {
                steps.push(other_steps[step]);
                steps.len() - 1
            });
        }
        Self { steps, products }
    }
}

#[derive(Default)]
pub struct Materials {
    materials: HashMap<TypeId, Box<dyn Any>>,
}

impl Materials {
    fn new() -> Self {
        Self::default()
    }

    fn get(&self, id: TypeId) -> Option<&dyn Any> {
        self.materials.get(&id).map(|b| &**b)
    }

    fn apply(&mut self, recipe: &dyn Transmutation) {
        let product_type = recipe.product();
        let ingredients: Vec<&dyn Any> = recipe
            .ingredients()
            .iter()
            .map(|&ingredient| self.get(ingredient).unwrap())
            .collect();
        let product = recipe.transmute(&ingredients);
        self.materials.insert(product_type, product);
    }

    fn into_material<T: 'static>(mut self) -> T {
        *self
            .materials
            .remove(&TypeId::of::<T>())
            .expect("material was not found")
            .downcast::<T>()
            .unwrap()
    }
}

impl<'a> FromIterator<&'a dyn Transmutation> for Materials {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = &'a dyn Transmutation>,
    {
        let mut materials = Self::new();
        for recipe in iter {
            materials.apply(recipe);
        }
        materials
    }
}
