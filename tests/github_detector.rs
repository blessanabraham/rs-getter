mod github_detector {
    use getter::{
        detect,
        detector::{Detector, GitHubDetector},
    };

    macro_rules! detect_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            # [tokio::test]
            async fn $name() {
                let _ = env_logger::builder().is_test(true).try_init();

                let (input, expected) = $value;

                let pwd = "/pwd";
                let detectors: Vec<Box<dyn Detector>> = vec![Box::new(GitHubDetector)];
                assert_eq !(expected, detect(input, pwd, &detectors).await.unwrap())
            }
        )*
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    detect_tests! {
        test_detect_1: ("github.com/blessanabraham/rs-getter",
        "git::https://github.com/blessanabraham/rs-getter.git"),
        test_detect_2: ("github.com/blessanabraham/rs-getter.git",
        "git::https://github.com/blessanabraham/rs-getter.git"),
        test_detect_3: ("github.com/blessanabraham/rs-getter/src",
        "git::https://github.com/blessanabraham/rs-getter.git//src"),
        test_detect_4: ("github.com/blessanabraham/rs-getter?foo=bar",
        "git::https://github.com/blessanabraham/rs-getter.git?foo=bar"),
        test_detect_5: ("github.com/blessanabraham/rs-getter.git?foo=bar",
        "git::https://github.com/blessanabraham/rs-getter.git?foo=bar"),
    }
}
