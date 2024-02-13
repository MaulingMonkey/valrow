// For e.g. debugger testing

#[cfg(not(feature = "alloc"))] fn main() { panic!("this example requires feature = \"alloc\"") }
#[cfg(    feature = "alloc" )] fn main() {
    let a = std::sync::Arc::new(42);
    let b = valrow::Valrow::new(&a);
    let c = &a;
    dbg!((&a, b, c));

    let mut d = ();
    let e = valrow::ValrowMut::new(&mut d);
    let e = &e; // VSC can't see ZSTs on the stack... but it *can* see *references* to them on the stack
    dbg!(e);

    let d = &d; // VSC can't see ZSTs on the stack... but it *can* see *references* to them on the stack
    dbg!(d);
}
