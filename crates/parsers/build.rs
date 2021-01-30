use std::path::Path;

fn compile_tree_sitter_grammars() {
    let dir = Path::new("../../vendor/tree-sitter-ddlog");

    println!("cargo:rerun-if-changed={:?}", dir.join("ddlog/dat/src/parser.c"));
    let mut cc = cc::Build::new();
    cc.include(dir.join("ddlog/dat/src"));
    cc.file(dir.join("ddlog/dat/src/parser.c"));
    cc.file(dir.join("ddlog/dat/src/scanner.cc"));
    cc.flag_if_supported("-Wno-maybe-uninitialized");
    cc.flag_if_supported("-Wno-unused-but-set-variable");
    cc.compile("tree-sitter-ddlog-dat");

    println!("cargo:rerun-if-changed={:?}", dir.join("ddlog/dl/src/parser.c"));
    let mut cc = cc::Build::new();
    cc.include(dir.join("ddlog/dl/src"));
    cc.file(dir.join("ddlog/dl/src/parser.c"));
    cc.file(dir.join("ddlog/dl/src/scanner.cc"));
    cc.flag_if_supported("-Wno-maybe-uninitialized");
    cc.flag_if_supported("-Wno-unused-but-set-variable");
    cc.compile("tree-sitter-ddlog-dl");
}

fn main() -> anyhow::Result<()> {
    if std::env::var("CARGO_CFG_TARGET_ARCH")? != "wasm32" {
        compile_tree_sitter_grammars();
    }
    Ok(())
}
