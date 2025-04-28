
fn main()  {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .input_extern_file("src/promise.rs")
        .input_extern_file("src/scheduler.rs")
        .input_extern_file("src/sleep.rs")
        .csharp_dll_name("Turso")
        .csharp_use_function_pointer(true) 
        .csharp_namespace("LibSql.DbConnection.Bindings")
        .csharp_class_accessibility("public")
        .generate_csharp_file("./bindings/NativeMethods.g.cs")
        .unwrap();
}