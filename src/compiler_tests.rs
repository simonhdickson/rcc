macro_rules! compiler_test {
    ( $test_name:ident, $file_name:expr, $expected_success:expr ) => {
        #[test]
        fn $test_name() {
            use ast;
            use lex;

            let content: &'static str = include_str!($file_name);

            let tokens = lex::parse(content);

            let program = ast::parse(tokens);

            if $expected_success {
                assert!(program.is_ok(), format!("expected ok, got {:#?}", program));
            } else {
                assert!(program.is_err(), format!("expected err, got {:#?}", program));
            }
        }
    };
}

compiler_test!(constant_only, "../c_programs/constant_only.c", true);
