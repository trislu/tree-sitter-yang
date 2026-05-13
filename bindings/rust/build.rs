fn main() {
    #[cfg(feature = "dev")]
    {
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

        use codegen::Scope;
        use convert_case::{Case, Casing};
        use serde::{
            Deserialize, Serialize,
            de::{DeserializeOwned, Error},
        };
        use std::{
            fs::{File, read_to_string},
            io::Write,
            path::Path,
        };

        #[derive(Serialize, Deserialize)]
        struct NodeType {
            #[serde(alias = "type")]
            type_name: String,
            named: bool,
        }

        // rust codegen
        fn read_to_json<T>(path: &Path) -> Result<T, impl Error>
        where
            T: DeserializeOwned,
        {
            let json_str =
                read_to_string(path).unwrap_or_else(|_| panic!("Failed to read file: {:?}", path));
            serde_json::from_str::<T>(&json_str)
        }

        let nodetype_json_path = Path::new("src/node-types.json");
        println!(
            "cargo:rerun-if-changed={}",
            nodetype_json_path.to_str().unwrap()
        );
        let nodetype_json: Result<Vec<NodeType>, _> = read_to_json(nodetype_json_path);

        let mut yang_scope = Scope::new();
        yang_scope.import("strum_macros", "Display");
        yang_scope.import("strum_macros", "EnumCount");
        yang_scope.import("strum_macros", "EnumIter");
        yang_scope.import("strum_macros", "EnumString");

        // enum for statement kind
        let enum_rule = yang_scope
            .new_enum("StatementKind")
            .vis("pub")
            .derive("Clone")
            .derive("Copy")
            .derive("Debug")
            .derive("Display")
            .derive("EnumCount")
            .derive("EnumIter")
            .derive("EnumString")
            .derive("Eq")
            .derive("Hash")
            .derive("PartialEq");

        for node_type in nodetype_json.unwrap() {
            if !node_type.type_name.ends_with("_stmt") {
                // only generate enum variants for statement nodes,
                // which are the ones we care about for the public API.
                // The other nodes are mostly internal and not useful to expose.
                continue;
            }
            let name = node_type.type_name;
            let name_without_stmt = name.replace("_stmt", "");
            enum_rule
                .new_variant(name_without_stmt.to_case(Case::Pascal))
                .annotation(format!(
                    r#"#[strum(to_string = "{}", serialize = "{}")]"#,
                    name_without_stmt.replace("_", "-"),
                    name
                ));
        }
        // code gen for yang.rs
        let yang_rs_path = Path::new("bindings")
            .join("rust")
            .join("yang")
            .join("statement_generated.rs");
        let mut yang_rs = File::create(yang_rs_path).unwrap();
        yang_rs
            .write_all(yang_scope.to_string().as_bytes())
            .unwrap();
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
