mod core;

fn main() {
    let _core = core::CoreBuilder::new()
        .build()
        .expect("Cannot Create Core!")
        .run()
        .expect("Core Exitted!");
}
