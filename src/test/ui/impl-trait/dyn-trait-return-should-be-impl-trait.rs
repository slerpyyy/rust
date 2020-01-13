#![allow(bare_trait_objects)]
struct Struct;
trait Trait {}
impl Trait for Struct {}
impl Trait for u32 {}

fn fuz() -> (usize, Trait) { (42, Struct) }
//~^ ERROR E0277
//~| ERROR E0308
fn bar() -> (usize, dyn Trait) { (42, Struct) }
//~^ ERROR E0277
//~| ERROR E0308
fn bap() -> Trait { Struct }
//~^ ERROR E0746
//~| ERROR E0308
fn ban() -> dyn Trait { Struct }
//~^ ERROR E0746
//~| ERROR E0308
fn bak() -> dyn Trait { unimplemented!() } //~ ERROR E0277
// Suggest using `Box<dyn Trait>`
fn bal() -> dyn Trait { //~ ERROR E0746
    if true {
        return Struct; //~ ERROR E0308
    }
    42 //~ ERROR E0308
}

// Suggest using `impl Trait`
fn bat() -> dyn Trait { //~ ERROR E0746
    if true {
        return 0; //~ ERROR E0308
    }
    42 //~ ERROR E0308
}

fn main() {}
