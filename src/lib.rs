use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::iter::FromIterator;

/// Transmutations require ingredients and produce a product.
pub trait Transmutation {
    fn ingredients(&self) -> &'static [TypeId];
    fn product(&self) -> TypeId;
    fn transmute(&self, inputs: &[&dyn Any]) -> Box<dyn Any>;
}

#[macro_export]
macro_rules! circle {
    (($($arg_name:tt $arg_colon:tt &$arg_ty:ty),*) -> $return_ty:ty $body:block) => {{
        paste::expr! {
            {
                use std::any::{Any, TypeId};
                struct Temporary<F>(F);
                const TEMPORARY_INGREDIENTS: &[TypeId] = &[$(TypeId::of::<$arg_ty>()),*];
                impl<F: Fn($(&$arg_ty),*) -> $return_ty> Transmutation for Temporary<F> {
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
                Temporary(|$($arg_name $arg_colon &$arg_ty),*| -> $return_ty {$body})
            }
        }
    }}
}

/// This is where all of the transmutation circles are inscribed.
#[derive(Default)]
pub struct Tome {
    /// Transmutation circles are organized by their products in the tomb.
    circles: HashMap<TypeId, Vec<Box<dyn Transmutation>>>,
}

impl Tome {
    pub fn new() -> Self {
        Self::default()
    }

    /// Inscribe a note about a natural transmutation into the tome.
    pub fn inscribe<T: Transmutation + 'static>(&mut self, circle: T) {
        let product_circles = self.circles.entry(circle.product()).or_default();
        product_circles.push(Box::new(circle));
        product_circles.sort_by_key(|c| c.ingredients().len());
    }

    /// Give me what I want.
    pub fn summon<T: 'static>(&self) -> Option<T> {
        // Find a recipe to create the item. This may fail.
        let recipe: Recipe = self.research::<T>()?;
        eprintln!("steps: {}", recipe.steps.len());
        // Perform the whole recipe. This cannot fail, excpet via panic.
        let materials: Materials = recipe.steps.into_iter().collect();
        eprintln!("materials: {}", materials.materials.len());
        // Drop all the intermediate materials to get only the desired one.
        Some(materials.into_material::<T>())
    }

    /// Give me what I want, but inscribe the process so it can be more quickly executed in the future.
    pub fn preserve<T>(&self) -> Option<T> {
        unimplemented!()
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
struct Materials {
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
