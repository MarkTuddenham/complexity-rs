use tree_sitter::Language;

extern "C" {
    pub fn tree_sitter_rust() -> Language;
    // pub fn tree_sitter_c() -> Language;
    // pub fn tree_sitter_javascript() -> Language;
}

pub fn get_language(lang: &str) -> Result<Language, &str> {
    match lang {
        "rust" => Ok(unsafe { tree_sitter_rust() }),
        _ => Err("No such language"),
    }
}
