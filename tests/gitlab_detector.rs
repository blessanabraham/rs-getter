mod gitlab_detector {
    use getter::{
        detect,
        detector::{Detector, GitLabDetector},
    };

    macro_rules! detect_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            # [tokio::test]
            async fn $name() {
                let _ = env_logger::builder().is_test(true).try_init();

                let (input, expected) = $value;

                let pwd = "/pwd";
                let detectors: Vec<Box<dyn Detector>> = vec![Box::new(GitLabDetector)];
                assert_eq !(expected, detect(input, pwd, &detectors).await.unwrap())
            }
        )*
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    detect_tests! {
        test_detect_1: (
            "gitlab.com/blessanabraham/foo",
            "git::https://gitlab.com/blessanabraham/foo.git",
        ),
        test_detect_2: (
            "gitlab.com/blessanabraham/foo.git",
            "git::https://gitlab.com/blessanabraham/foo.git",
        ),
        test_detect_3: (
            "gitlab.com/blessanabraham/foo/bar",
            "git::https://gitlab.com/blessanabraham/foo.git//bar",
        ),
        test_detect_4: (
            "gitlab.com/blessanabraham/foo?foo=bar",
            "git::https://gitlab.com/blessanabraham/foo.git?foo=bar",
        ),
        test_detect_5: (
            "gitlab.com/blessanabraham/foo.git?foo=bar",
            "git::https://gitlab.com/blessanabraham/foo.git?foo=bar",
        ),
    }
}
