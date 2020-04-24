#![feature(const_type_id)]

use summon::{circle, fusion, Tome, Transmutation};

#[derive(Clone)]
struct A;

#[derive(Clone)]
struct B;

#[derive(Debug)]
struct C;

#[test]
fn fuse() {
    let mut tome = Tome::new();
    tome.ether(A);
    tome.ether(B);
    tome.inscribe(fusion!((A, B) -> C));
    println!("{:?}", tome.summon::<C>().unwrap());
}

#[derive(Clone)]
struct ConstantAcceleration(f64);
#[derive(Clone)]
struct InitialVelocity(f64);
#[derive(Clone)]
struct InitialPosition(f64);
#[derive(Clone)]
struct Time(f64);

struct Distance(f64);

#[test]
fn sum() {
    let mut tome = Tome::new();
    tome.ether(ConstantAcceleration(3.0));
    tome.ether(InitialVelocity(5.0));
    tome.ether(InitialPosition(6.0));
    tome.ether(Time(4.0));
    tome.inscribe(
        circle!((a: &ConstantAcceleration, v: &InitialVelocity, p: &InitialPosition, t: &Time) -> Distance {
            Distance(0.5 * a.0 * t.0.powi(2) + v.0 * t.0 + p.0)
        }),
    );
    assert_eq!(
        0.5 * 3.0 * 4.0f64.powi(2) + 5.0 * 4.0 + 6.0,
        tome.summon::<Distance>().unwrap().0
    );
}
