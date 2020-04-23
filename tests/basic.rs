#![feature(const_type_id)]

use summon::*;

struct A;

struct B;

#[derive(Debug)]
struct C;

#[test]
fn addition() {
    let mut tome = Tome::new();
    tome.inscribe(circle!(() -> A { A }));
    tome.inscribe(circle!(() -> B { B }));
    tome.inscribe(circle!((a: &A, b: &B) -> C { C }));
    println!("{:?}", tome.summon::<C>().unwrap());
}