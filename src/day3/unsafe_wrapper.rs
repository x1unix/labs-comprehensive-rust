use super::ffi::{self, dirent};

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        // Call opendir and return a Ok value if that worked,
        // otherwise return Err with a message.
        let iter = unsafe {
            let cpath =
                CString::new(path).map_err(|err| format!("can't prepare CString: {err}"))?;

            let dir = ffi::opendir(cpath.as_ptr());
            DirectoryIterator { path: cpath, dir }
        };

        if iter.dir.is_null() {
            return Err(format!("can't open {path}"));
        }

        Ok(iter)
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        // Keep calling readdir until we get a NULL pointer back.
        unsafe {
            let dirent_ptr = ffi::readdir(self.dir);
            if dirent_ptr.is_null() {
                return None;
            }

            let dirent {
                d_namlen, d_name, ..
            } = *dirent_ptr;
            let name = CStr::from_ptr(d_name.as_ptr());
            Some(OsStr::from_bytes(name.to_bytes()).to_owned())
        }
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        // Call closedir as needed.
        unsafe {
            if !self.dir.is_null() {
                ffi::closedir(self.dir);
            }
        }
    }
}

mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_main() -> Result<(), String> {
        let iter = DirectoryIterator::new(".")?;
        println!("files: {:#?}", iter.collect::<Vec<_>>());
        Ok(())
    }

    #[test]
    fn test_nonexisting_directory() {
        let iter = DirectoryIterator::new("no-such-directory");
        assert!(iter.is_err());
    }

    #[test]
    fn test_empty_directory() -> Result<(), Box<dyn Error>> {
        let tmp = tempfile::TempDir::new()?;
        let iter =
            DirectoryIterator::new(tmp.path().to_str().ok_or("Non UTF-8 character in path")?)?;
        let mut entries = iter.collect::<Vec<_>>();
        entries.sort();
        assert_eq!(entries, &[".", ".."]);
        Ok(())
    }

    #[test]
    fn test_nonempty_directory() -> Result<(), Box<dyn Error>> {
        let tmp = tempfile::TempDir::new()?;
        std::fs::write(tmp.path().join("foo.txt"), "The Foo Diaries\n")?;
        std::fs::write(tmp.path().join("bar.png"), "<PNG>\n")?;
        std::fs::write(tmp.path().join("crab.rs"), "//! Crab\n")?;
        let iter =
            DirectoryIterator::new(tmp.path().to_str().ok_or("Non UTF-8 character in path")?)?;
        let mut entries = iter.collect::<Vec<_>>();
        entries.sort();
        assert_eq!(entries, &[".", "..", "bar.png", "crab.rs", "foo.txt"]);
        Ok(())
    }
}
