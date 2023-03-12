use std::path::PathBuf;

fn main() {
    let languages = ["rust"];

    languages.iter().for_each(|language| {
        build_sitter(language);
    });
}

fn build_sitter(language: &str) {
    let package = format!("tree-sitter-{language}");
    let dir: PathBuf = ["vendor", &package, "src"].iter().collect();

    let mut builder = cc::Build::new();
    builder.include(&dir);

    // we don't maintain the TS projects so we don't care about the warnings
    builder.warnings(false).extra_warnings(false);

    let source_files = ["parser.c", "scanner.c"];
    for file in source_files {
        let file_path = dir.join(file);
        builder.file(file_path.clone());

        // Only rebuild if one of these files changes
        println!("cargo:rerun-if-changed={}", file_path.to_str().unwrap());
    }

    builder.compile(&package);
}
