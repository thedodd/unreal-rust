use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .include_item("UnrealBindings")
        .include_item("RustBindings")
        .include_item("CreateRustBindings")
        .include_item("EntryUnrealBindingsFn")
        .include_item("EntryBeginPlayFn")
        .include_item("EntryTickFn")
        .include_item("RetrieveUuids")
        .with_pragma_once(true)
        //.with_config(Config {
        //    structure: StructConfig  {
        //        derive_constructor: true,
        //        ..Default::default()
        //    },
        //    ..Default::default()
        //})
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("../RustPlugin/Source/RustPlugin/Public/Bindings.h");
}