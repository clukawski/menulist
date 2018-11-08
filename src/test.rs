use super::menu;
use std::{fs, io, panic};

#[test]
fn expand_output_test() -> io::Result<()> {
    run_test(|| {
        let exts = vec!["txt", "csv"];
        let mut output = String::new();

        let _generated =
            menu::expand_path(Some(String::from("/tmp/vmenu_test/")), &exts, &mut output);

        // We're not looking for .mp4 files
        assert_eq!(output.find("/tmp/vmenu_test/test.mp4"), None);
        // Ensure output is as expected (newer files first)
        assert_eq!("/tmp/vmenu_test/newer.csv\n/tmp/vmenu_test/test.csv\n/tmp/vmenu_test/test.txt\n/tmp/vmenu_test/2/test2.txt", output.trim());
        Ok(())
    })
}

fn setup() -> io::Result<()> {
    fs::create_dir("/tmp/vmenu_test/")?;
    fs::create_dir("/tmp/vmenu_test/2/")?;
    // Open a file in write-only mode, returns `io::Result<File>`
    fs::File::create("/tmp/vmenu_test/test.txt")?;
    fs::File::create("/tmp/vmenu_test/test.csv")?;
    fs::File::create("/tmp/vmenu_test/newer.csv")?;
    fs::File::create("/tmp/vmenu_test/test.mp4")?; // This file won't be listed
    fs::File::create("/tmp/vmenu_test/2/test2.txt")?;
    Ok(())
}

fn teardown() -> io::Result<()> {
    fs::remove_dir_all("/tmp/vmenu_test/")?;
    Ok(())
}

fn run_test<T>(test: T) -> io::Result<()>
where
    T: FnOnce() -> io::Result<()> + panic::UnwindSafe,
{
    setup()?;

    let result = panic::catch_unwind(|| test());

    teardown()?;

    assert!(result.is_ok());
    Ok(())
}
