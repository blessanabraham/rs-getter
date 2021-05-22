mod detect {
    use getter::{detect, DETECTORS};

    macro_rules! detect_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            # [tokio::test]
            async fn $name() {
                let _ = env_logger::builder().is_test(true).try_init();

                let (input, pwd, expected) = $value;

                assert_eq !(expected, detect(input, pwd, &DETECTORS).await.unwrap())
            }
        )*
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    detect_tests! {
        test_detect_1: (
            "./foo",
            "/opt",
            "file:///opt/foo",
        ),
        test_detect_2: (
            "git::./foo",
            "/opt",
            "git::file:///opt/foo",
        ),
        test_detect_3: (
            "git::github.com/blessanabraham/foo",
            "",
            "git::https://github.com/blessanabraham/foo.git",
        ),
        test_detect_4: (
            "./foo//bar",
            "/opt",
            "file:///opt/foo//bar",
        ),
        test_detect_5: (
            "git::github.com/blessanabraham/foo//bar",
            "",
            "git::https://github.com/blessanabraham/foo.git//bar",
        ),
        test_detect_6: (
            "git::https://github.com/blessanabraham/rs-getter.git",
            "",
            "git::https://github.com/blessanabraham/rs-getter.git",
        ),
        test_detect_7: (
            "git::https://person@someothergit.com/foo/bar",
            "",
            "git::https://person@someothergit.com/foo/bar",
        ),
        test_detect_8: (
            "git::https://person@someothergit.com/foo/bar",
            "/opt",
            "git::https://person@someothergit.com/foo/bar",
        ),
        test_detect_9: (
            "./foo/archive//*",
            "/opt",
            "file:///opt/foo/archive//*",
        ),
        test_detect_10: (
            "git::ssh://git@my.custom.git/dir1/dir2",
            "",
            "git::ssh://git@my.custom.git/dir1/dir2",
        ),
        test_detect_11: (
            "git::git@my.custom.git:dir1/dir2",
            "/opt",
            "git::ssh://git@my.custom.git/dir1/dir2",
        ),
        test_detect_12: (
            "git::git@my.custom.git:dir1/dir2",
            "",
            "git::ssh://git@my.custom.git/dir1/dir2",
        ),
    }
}
