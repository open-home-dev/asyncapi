use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use regex::Regex;
use std::{
    env::var,
    fs::{read_dir, File},
    io::{self, Write},
    path::Path,
    process::Command,
};

fn main() {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    let curr_dir = Path::new(&manifest_dir);
    let tck_path = Path::new(curr_dir).join("fixtures").join("tck");
    Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .current_dir(&tck_path)
        .status()
        .unwrap();

    let mut test_file = File::create(curr_dir.join("tests").join("tck.rs")).unwrap();

    writeln!(
        test_file,
        "{}",
        quote! {
            use asyncapi::{AsyncAPI};
        }
        .to_string()
    )
    .unwrap();

    let re = Regex::new(r"[^A-Za-z0-9]").unwrap();

    find_test(&tck_path.join("tests"), &mut test_file, &re).unwrap();
}

fn find_test(dir: &Path, file: &mut File, ident_regex: &Regex) -> io::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                find_test(&path, file, ident_regex)?;
            } else {
                let valid = if entry.file_name().to_string_lossy().starts_with("invalid") {
                    false
                } else if entry.file_name().to_string_lossy().starts_with("valid") {
                    true
                } else {
                    continue;
                };
                let path_string = path.to_string_lossy();

                if valid {
                    let test = quote! {
                        let _asyncapi: AsyncAPI = serde_yaml::from_str(include_str!(#path_string))
                        .expect(&format!("Could not deserialize {}", #path_string));
                    };
                    let test_name = format_ident!(
                        "test_{}",
                        ident_regex
                            .replace_all(&path_string, "_")
                            .to_case(Case::Snake)
                    );
                    let test = quote! {
                        #[test]
                        fn #test_name() {
                            #test
                        }
                    };
                    writeln!(file, "{}", test.to_string()).unwrap();
                }
            }
        }
    }
    Ok(())
}
