
fn main()  {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("example")
        .generate_csharp_file("./bindings/NativeMethods.g.cs")
        .unwrap();
}