use std::io::Read;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    PathDoesNotExist,
    FileContainsNil,
}

impl From<std::io::Error> for Error {

    fn from(inp: std::io::Error) -> Self {
        return Error::Io(inp);
    }

}

pub fn read_as_cstring<P: AsRef<std::path::Path>>(
    path:   P
) -> Result<std::ffi::CString, Error> {

    let mut file = std::fs::File::open(path)?;
    let mut buffer: Vec<u8> = Vec::with_capacity(
        file.metadata()?.len() as usize + 1
    );
    file.read_to_end(&mut buffer)?;
    if buffer.iter().find(|c| **c == 0).is_some() {
        return Err(Error::FileContainsNil);
    }
    return Ok(unsafe {
        std::ffi::CString::from_vec_unchecked(buffer)
    });

}
