
fn main()  {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("Turso")
        .csharp_namespace("LibSql.DbConnection.Bindings")
        .generate_csharp_file("./bindings/NativeMethods.g.cs")
        .unwrap();
}