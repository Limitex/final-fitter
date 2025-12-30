use glob::glob;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

const PROTO_ROOT: &str = "proto";
const OUT_DIR: &str = "src/generated";

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let proto_root = Path::new(PROTO_ROOT);
    let out_dir = Path::new(OUT_DIR);

    let protos = collect_proto_files(proto_root)?;
    register_rerun_triggers(&protos);

    fs::create_dir_all(out_dir)?;
    compile_protos(&protos, proto_root.to_path_buf(), out_dir)?;
    generate_mod_file(out_dir)?;

    Ok(())
}

fn collect_proto_files(root: &Path) -> Result<Vec<PathBuf>> {
    let pattern = format!("{}/**/*.proto", root.display());
    let protos: Vec<_> = glob(&pattern)?.filter_map(|r| r.ok()).collect();

    if protos.is_empty() {
        return Err(format!("No .proto files found in {:?}", root).into());
    }

    Ok(protos)
}

fn register_rerun_triggers(protos: &[PathBuf]) {
    for proto in protos {
        println!("cargo::rerun-if-changed={}", proto.display());
    }
}

fn compile_protos(protos: &[PathBuf], include: PathBuf, out_dir: &Path) -> Result<()> {
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(out_dir)
        .compile_protos(protos, &[include])?;

    Ok(())
}

fn generate_mod_file(out_dir: &Path) -> Result<()> {
    let modules = collect_module_names(out_dir)?;
    let content = build_mod_content(&modules);
    fs::write(out_dir.join("mod.rs"), content)?;

    Ok(())
}

fn collect_module_names(dir: &Path) -> Result<Vec<String>> {
    let pattern = format!("{}/*.rs", dir.display());

    let mut modules: Vec<_> = glob(&pattern)?
        .filter_map(|r| r.ok())
        .filter_map(|path| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .filter(|&s| s != "mod")
                .map(String::from)
        })
        .collect();

    modules.sort();
    Ok(modules)
}

fn build_mod_content(modules: &[String]) -> String {
    modules
        .iter()
        .map(|filename| {
            let mod_name = filename.replace('.', "_");
            format!("#[path = \"{filename}.rs\"]\npub mod {mod_name};")
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}
