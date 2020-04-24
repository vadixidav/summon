#![feature(const_type_id, concat_idents)]

use summon::{circle, fusion, Tome, Transmutation};

struct A;

struct B;

#[derive(Debug)]
struct C;

#[test]
fn fuse() {
    let mut tome = Tome::new();
    tome.inscribe(fusion!(() -> A));
    tome.inscribe(fusion!(() -> B));
    tome.inscribe(fusion!((A, B) -> C));
    println!("{:?}", tome.summon::<C>().unwrap());
}
