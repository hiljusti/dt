use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=dt-src");
    copy_dt_libs();
}

fn copy_dt_libs() {
    let from = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("dt-src")
        .join("stdlib");
    let to = Path::new(&env::var("OUT_DIR").unwrap()).join("stdlib");

    fs::create_dir(to.clone()).unwrap_or(());

    let lib_files = fs::read_dir(from)
        .expect("Could not find dt stdlib")
        .collect::<Vec<_>>();

    assert_ne!(0, lib_files.len());

    lib_files
        .into_iter()
        .map(|dir_entry| dir_entry.unwrap())
        .for_each(|file| {
            let from = file.path();

            let file_name = from.file_name().unwrap();
            let to = to.join(file_name);

            fs::copy(from, to).unwrap();
        });
}
