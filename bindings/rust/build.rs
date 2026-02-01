fn main() {
    let tree_sitter_available = std::process::Command::new("tree-sitter")
        .arg("--version")
        .status()
        .is_ok();

    if tree_sitter_available {
        let output = std::process::Command::new("tree-sitter")
            .arg("generate")
            .arg("--abi=14")
            .output()
            .expect("Failed to execute tree-sitter build command");

        if !output.status.success() {
            let error_message = String::from_utf8_lossy(&output.stderr);
            panic!("Tree-sitter build failed: {error_message}");
        }
    }

    let src_dir = std::path::Path::new("src");

    let mut c_config = cc::Build::new();
    c_config.std("c11").include(src_dir);

    #[cfg(target_env = "msvc")]
    c_config.flag("-utf-8");

    if std::env::var("TARGET").unwrap() == "wasm32-unknown-unknown" {
        let Ok(wasm_headers) = std::env::var("DEP_TREE_SITTER_LANGUAGE_WASM_HEADERS") else {
            panic!(
                "Environment variable DEP_TREE_SITTER_LANGUAGE_WASM_HEADERS must be set by the language crate"
            );
        };
        let Ok(wasm_src) =
            std::env::var("DEP_TREE_SITTER_LANGUAGE_WASM_SRC").map(std::path::PathBuf::from)
        else {
            panic!(
                "Environment variable DEP_TREE_SITTER_LANGUAGE_WASM_SRC must be set by the language crate"
            );
        };

        c_config.include(&wasm_headers);
        c_config.files([
            wasm_src.join("stdio.c"),
            wasm_src.join("stdlib.c"),
            wasm_src.join("string.c"),
        ]);
    }

    let parser_path = src_dir.join("parser.c");
    c_config.file(&parser_path);
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());

    let scanner_path = src_dir.join("scanner.c");

    c_config.file(&scanner_path);
    println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());

    c_config.compile("tree-sitter-yang");
}
