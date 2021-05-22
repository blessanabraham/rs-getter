mod bitbucket_detector {
    use getter::{
        detect,
        detector::{BitBucketDetector, Detector},
    };

    macro_rules! detect_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            # [tokio::test]
            async fn $name() {
                let _ = env_logger::builder().is_test(true).try_init();

                let (input, expected) = $value;

                let pwd = "/pwd";
                let detectors: Vec<Box<dyn Detector>> = vec![Box::new(BitBucketDetector)];
                assert_eq!(expected, detect(input, pwd, &detectors).await.unwrap())
            }
        )*
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    detect_tests! {
        test_detect_1: ("bitbucket.org/hashicorp/tf-test-git",
            "git::https://bitbucket.org/hashicorp/tf-test-git.git"),
        test_detect_2: ("bitbucket.org/hashicorp/tf-test-git.git",
            "git::https://bitbucket.org/hashicorp/tf-test-git.git"),
    }
}
