#[cfg(not(target_arch = "wasm32"))]
mod file_detector {
    use getter::detector::{Detector, FileDetector};
    use std::env;
    use std::fs::DirBuilder;
    use tempdir::TempDir;

    mod general {
        use super::*;

        macro_rules! detect_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[tokio::test]
                async fn $name() {
                    let _ = env_logger::builder().is_test(true).try_init();

                    let (input, pwd, expected) = $value;

                    let pwd_root = env::current_dir().unwrap().canonicalize().unwrap();

                    let expected = if !expected.starts_with("file://") {
                        format!("file://{}/{}", pwd_root.to_str().unwrap(), expected)
                    } else {
                        expected.to_string()
                    };

                    let detector = FileDetector;
                    let (out, ok) = detector.detect(input, &pwd).await.unwrap();

                    assert!(ok, "not ok");

                    assert_eq!(out, expected);
                }
            )*
            }
        }

        macro_rules! no_pwd_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[tokio::test]
                async fn $name() {
                    let _ = env_logger::builder().is_test(true).try_init();

                    let (input, pwd, expected) = $value;

                    let detector = FileDetector;
                    let (out, ok) = detector.detect(input, &pwd).await.unwrap();

                    assert!(ok, "not ok");

                    assert_eq!(out, expected);
                }
            )*
            }
        }

        macro_rules! no_pwd_panic_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[should_panic]
                #[tokio::test]
                async fn $name() {
                    let _ = env_logger::builder().is_test(true).try_init();

                    let (input, pwd, expected) = $value;

                    let detector = FileDetector;
                    let (out, ok) = detector.detect(input, &pwd).await.unwrap();

                    assert!(ok, "not ok");

                    assert_eq!(out, expected);
                }
            )*
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        detect_tests! {
            test_detect_1: ("./foo", "/opt", "file:///opt/foo"),
            test_detect_2: ("./foo?foo=bar", "/opt", "file:///opt/foo?foo=bar"),
            test_detect_3: ("foo", "/opt", "file:///opt/foo"),
        }

        #[cfg(all(target_family = "windows", not(target_arch = "wasm32")))]
        detect_tests! {
            test_detect_windows_1: ("/foo", "/opt", "file:///opt/foo"),
            test_detect_windows_2: (r#"C:\"#, "/opt", "file://C:/"),
            test_detect_windows_3: (r#"C:\?bar=baz"#, "/opt", "ile://C:/?bar=baz"),
        }

        #[cfg(all(target_family = "unix", not(target_arch = "wasm32")))]
        detect_tests! {
            test_detect_unix_1: (
                "./foo",
                "testdata/detect-file-symlink-pwd/syml/pwd",
                "testdata/detect-file-symlink-pwd/real/foo",
            ),
            test_detect_unix_2: ("/foo", "/opt", "file:///foo"),
            test_detect_unix_3: ("/foo?bar=baz", "/opt", "file:///foo?bar=baz"),
        }

        #[cfg(not(target_arch = "wasm32"))]
        no_pwd_panic_tests! {
            test_no_pwd_panic_1: ("./foo", "", ""),
            test_no_pwd_panic_2: ("foo", "", ""),
        }

        #[cfg(all(target_family = "windows", not(target_arch = "wasm32")))]
        no_pwd_panic_tests! {
            test_no_pwd_win_panic_1: ("/opt", "", ""),
        }

        #[cfg(all(target_family = "windows", not(target_arch = "wasm32")))]
        now_pwd_tests! {
            test_no_pwd_win_1: (in: r#"C:\"#, pwd: "", out: "file://C:/"),
        }

        #[cfg(all(target_family = "unix", not(target_arch = "wasm32")))]
        no_pwd_tests! {
            test_no_pwd_unix_1: ("/opt", "", "file:///opt"),
        }
    }

    #[cfg(all(target_family = "unix", not(target_arch = "wasm32")))]
    mod unix {
        use super::*;
        use std::os::unix::fs::{symlink, DirBuilderExt};

        #[tokio::test]
        async fn relative_symlink() {
            let _ = env_logger::builder().is_test(true).try_init();

            let tmp_dir = TempDir::new("rs-getter").unwrap();

            // We may have a symlinked tmp dir,
            // e.g. OSX uses /var -> /private/var
            let abs_temp_dir = tmp_dir.path().canonicalize().unwrap();

            let mut builder = DirBuilder::new();
            builder.mode(0o755);
            builder.create(abs_temp_dir.join("realPWD")).unwrap();

            let subdir = abs_temp_dir.join("subdir");
            builder.create(&subdir).unwrap();

            let prev_dir = env::current_dir().unwrap();

            env::set_current_dir(&subdir).unwrap();
            symlink("../realPWD", "linkedPWD").unwrap();

            // if detect doesn't fully resolve the pwd symlink, the output will be the
            // invalid path: "file:///../modules/foo"
            let detector = FileDetector;
            let (out, ok) = detector
                .detect("../modules/foo", "./linkedPWD")
                .await
                .unwrap();

            assert!(ok, "not ok");
            let expected = format!(
                "file://{}",
                abs_temp_dir.join("modules/foo").to_str().unwrap()
            );
            assert_eq!(out, expected);

            env::set_current_dir(prev_dir).unwrap();
        }
    }
}
