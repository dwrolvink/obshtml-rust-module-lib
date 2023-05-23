use super::errors::{Error};

// POSIX PATHS
// ===============================================================================
/*
    let abspath = AbsolutePosixPath::new("/path/to/file.json".to_string());
    println!("{}", abspath.to_string());
*/

pub struct RelativePosixPath(pub String);
impl RelativePosixPath {
    pub fn new(path: String) -> RelativePosixPath {
        // tests here:
        // *
        // return if okay
        return RelativePosixPath(path);
    }
    // pub fn to_absolute(root: AbsolutePosixPath) -> RelativePosixPath {
    //     // ??
    // }
    pub fn to_string(&self) -> String {
        return self.0.clone();
    }
}

pub struct AbsolutePosixPath(pub String);
impl AbsolutePosixPath 
{
    pub fn new(path: String) -> Result<AbsolutePosixPath, Error> 
    {
        if !path.starts_with('/') {
            return Err(Error::WrongPathType(
                format!("Path \"{}\" is not absolute!", path.as_str())
            ));
        }
        return Ok(AbsolutePosixPath(path));
    }
    // pub fn to_relative(root: AbsolutePosixPath) -> RelativePosixPath {
    //     // ??
    // }

    pub fn to_string(&self) -> String {
        return self.0.clone();
    }
}

pub enum PosixPath {
    RelativePosixPath(RelativePosixPath),
    AbsolutePosixPath(AbsolutePosixPath),
}
impl PosixPath {
    pub fn to_string(&self) -> String {
        return match self {
            PosixPath::RelativePosixPath(value) => value.to_string(),
            PosixPath::AbsolutePosixPath(value) => value.to_string(),
        }
    }
}

