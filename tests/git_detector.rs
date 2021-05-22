mod git_detector {
    use getter::{
        detect,
        detector::{Detector, GitDetector},
    };

    macro_rules! detect_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            # [tokio::test]
            async fn $name() {
                let _ = env_logger::builder().is_test(true).try_init();

                let (input, expected) = $value;

                let pwd = "/pwd";
                let detectors: Vec<Box<dyn Detector>> = vec![Box::new(GitDetector)];
                assert_eq !(expected, detect(input, pwd, &detectors).await.unwrap())
            }
        )*
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    detect_tests! {
        test_detect_1: ("git@github.com:blessanabraham/foo.git",
        "git::ssh://git@github.com/blessanabraham/foo.git"),
        test_detect_2: ("git@github.com:org/project.git?ref=test-branch",
        "git::ssh://git@github.com/org/project.git?ref=test-branch"),
        test_detect_3: ("git@github.com:blessanabraham/foo.git//bar",
        "git::ssh://git@github.com/blessanabraham/foo.git//bar"),
        test_detect_4: ("git@github.com:blessanabraham/foo.git?foo=bar",
        "git::ssh://git@github.com/blessanabraham/foo.git?foo=bar"),
        test_detect_5: ("git@github.xyz.com:org/project.git",
        "git::ssh://git@github.xyz.com/org/project.git"),
        test_detect_6: ("git@github.xyz.com:org/project.git?ref=test-branch",
        "git::ssh://git@github.xyz.com/org/project.git?ref=test-branch"),
        test_detect_7: ("git@github.xyz.com:org/project.git//module/a",
        "git::ssh://git@github.xyz.com/org/project.git//module/a"),
        test_detect_8: ("git@github.xyz.com:org/project.git//module/a?ref=test-branch",
        "git::ssh://git@github.xyz.com/org/project.git//module/a?ref=test-branch"),
        // Already in the canonical form, so no rewriting required
            // When the ssh: protocol is used explicitly, we recognize it as
            // URL form rather than SCP-like form, so the part after the colon
            // is a port number, not part of the path.
        test_detect_9: ("git::ssh://git@git.example.com:2222/hashicorp/foo.git",
        "git::ssh://git@git.example.com:2222/hashicorp/foo.git"),
    }
}
