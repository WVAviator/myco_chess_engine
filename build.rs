use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    let target_dir = Path::new(&out_dir)
        .ancestors()
        .nth(3)
        .expect("Failed to find target directory");

    let source_db = Path::new("resources/myco.db3");
    let source_model = Path::new("resources/myco_eval_model.pt");

    let dest_resources_dir = target_dir.join("resources");

    fs::create_dir_all(&dest_resources_dir).expect("Failed to create resources directory");

    if source_db.exists() {
        fs::copy(source_db, dest_resources_dir.join("myco.db3"))
            .expect("Failed to copy database file");
    }

    if source_model.exists() {
        fs::copy(source_model, dest_resources_dir.join("myco_eval_model.pt"))
            .expect("Failed to copy pytorch model file");
    }
}
