
fn main(){
    cc::Build::new()
        .files(&[
            "clox/chunk.c",
            "clox/debug.c",
            "clox/memory.c",
            "clox/scanner.c",
            "clox/value.c",
            "clox/compiler.c",
            "clox/object.c",
            "clox/table.c",
            "clox/vm.c",
            "clox/interface.c",
        ])
        .compile("libvm.a")

}
