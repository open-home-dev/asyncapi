use convert_case::{Case, Casing};
use quote::{format_ident, quote};
use regex::Regex;
use std::{
    env::var,
    ffi::OsStr,
    fs::{read_dir, File},
    io::{self, Write},
    path::Path,
    process::Command,
};

fn main() -> io::Result<()> {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();
    println!("{manifest_dir}");
    let curr_dir = Path::new(&manifest_dir);
    // tck tests
    let tck_path = Path::new(curr_dir).join("..").join("fixtures").join("tck");
    Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .current_dir(&tck_path)
        .status()
        .unwrap();

    let mut test_file = File::create(curr_dir.join("..").join("tests").join("tck.rs")).unwrap();

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

    // spec examples
    let spec_path = Path::new(curr_dir).join("..").join("fixtures").join("spec");
    Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .current_dir(&spec_path)
        .status()
        .unwrap();

    let mut examples_file =
        File::create(curr_dir.join("..").join("tests").join("spec-examples.rs"))?;

    writeln!(
        examples_file,
        "{}",
        quote! {
            use asyncapi::{AsyncAPI};
        }
        .to_string()
    )
    .unwrap();

    find_spec_example(&spec_path.join("examples"), &mut examples_file, &re)?;

    Ok(())
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
                    write_deserialize_test(&path_string, file, ident_regex)?;
                }
            }
        }
    }
    Ok(())
}

fn find_spec_example(examples_dir: &Path, file: &mut File, ident_regex: &Regex) -> io::Result<()> {
    for entry in examples_dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        let path_string = path.to_string_lossy();
        if entry.file_name().to_string_lossy().ends_with(".yml") {
            write_deserialize_test(&path_string, file, ident_regex)?
        }
    }
    let social_media_dir = examples_dir.join("social-media");
    for entry in social_media_dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            for entry in path.read_dir()? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() && path.file_name() == Some(OsStr::new("asyncapi.yaml")) {
                    let path_string = path.to_string_lossy();
                    write_deserialize_test(&path_string, file, ident_regex)?;
                }
            }
        }
    }
    Ok(())
}

fn write_deserialize_test(
    path_string: &str,
    file: &mut File,
    ident_regex: &Regex,
) -> io::Result<()> {
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
    writeln!(file, "{}", test.to_string())?;
    Ok(())
}
