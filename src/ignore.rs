use std::path::PathBuf;

pub fn ignore_path(path: &PathBuf) -> Result<(), std::io::Error> {
    xattr::set(path, "user.com.dropbox.ignored", "1".as_bytes())
}

pub fn unignore_path(path: &PathBuf) -> Result<(), std::io::Error> {
    check_path(path).and_then(|has_attr| {
        if has_attr {
            xattr::remove(path, "user.com.dropbox.ignored")
        } else {
            Ok(())
        }
    })
}

pub fn check_path(path: &PathBuf) -> Result<bool, std::io::Error> {
    xattr::get(path, "user.com.dropbox.ignored").map(|val| val.is_some())
}
