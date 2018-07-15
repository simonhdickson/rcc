extern crate rcc;

macro_rules! compiler_test_valid {
    ( $test_name:ident, $content:expr ) => {
        #[test]
        fn $test_name() {
            use std::env;
            use std::fs::{self, File};
            use std::io::prelude::*;
            use std::process::Command;

            let mut dir = env::temp_dir();
            dir.push(format!("{}.c", stringify!($test_name)));
            let source_path = dir.to_str().unwrap();
            let out_path = &format!("./{}.out", stringify!($test_name));
            let asm_path = &format!("{}.s", stringify!($test_name));
            
            let mut f = File::create(source_path).unwrap();
            f.write_all($content.as_bytes()).expect("something went wrong writing the file");

            println!("{}", 0);

            Command::new("gcc")
                .arg("-w")
                .arg(source_path)
                .arg("-o")
                .arg(out_path)
                .output()
                .expect("failed to execute process");
            println!("{}", 1);

            let expected_out =
                Command::new(out_path)
                    .status()
                    .expect("failed to execute process");
            println!("{}", 2);

            fs::remove_file(out_path).unwrap();

            Command::new("cargo")
                .arg("run")
                .arg("--release")
                .arg("--")
                .arg(source_path)
                .output()
                .expect("failed to execute process");
            println!("{}", 3);
            
            Command::new("gcc")
                .arg("-m32")
                .arg(asm_path)
                .arg("-o")
                .arg(out_path)
                .output()
                .expect("failed to execute process");
            println!("{}", 4);

            let out =
                Command::new(out_path)
                    .status()
                    .expect("failed to execute process");
            println!("{}", 5);

            fs::remove_file(source_path).unwrap();
            fs::remove_file(asm_path).unwrap();
            fs::remove_file(out_path).unwrap();

            assert_eq!(expected_out, out, "expected {:#?}, got {:#?}", expected_out, out);
        }
    };
}

compiler_test_valid!(constant_only, r"
int main() {
    return 2;
}");


compiler_test_valid!(one_line, r"int main() { return 0; }");

compiler_test_valid!(not_zero, r"int main() {
    return !5;
}");

compiler_test_valid!(bitwise, r"int main() {
    return !12;
}");

compiler_test_valid!(nested_ops, r"int main() {
    return !-3;
}");
