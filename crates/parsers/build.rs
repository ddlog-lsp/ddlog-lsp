use std::path::Path;

fn compile_tree_sitter_grammars() -> anyhow::Result<()> {
    let dir = Path::new("../../vendor/tree-sitter-ddlog");

    println!("cargo:rerun-if-changed={:?}", dir.join("ddlog/dat/src/parser.c"));
    let mut cc = cc::Build::new();
    cc.include(dir.join("ddlog/dat/src"));
    cc.file(dir.join("ddlog/dat/src/parser.c"));
    cc.file(dir.join("ddlog/dat/src/scanner.cc"));
    cc.compile("tree-sitter-ddlog-dat");

    println!("cargo:rerun-if-changed={:?}", dir.join("ddlog/dl/src/parser.c"));
    let mut cc = cc::Build::new();
    cc.include(dir.join("ddlog/dl/src"));
    cc.file(dir.join("ddlog/dl/src/parser.c"));
    cc.file(dir.join("ddlog/dl/src/scanner.cc"));
    cc.compile("tree-sitter-ddlog-dl");

    Ok(())
}

fn main() -> anyhow::Result<()> {
    compile_tree_sitter_grammars()?;
    Ok(())
}
