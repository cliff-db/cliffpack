use std::fmt;
use std::io;
use std::string::FromUtf8Error;

/// This enum represents all possible errors when reading a plugin package.
#[derive(Debug)]
pub enum PluginReadError {
    /// This error occurs if the magic number is incorrect. The magic number uniquely identifies the cliff plugin file type (.cpk).
    InvalidMagicNumber,
    /// This error occurs if the file version of the plugin package does not match. This value exists to allow future updates to the format.
    NotSupportedFileVersion,
    /// This error occurs if the file is corrupted or not readable.
    CorruptedFile,
    /// This enum represents an IO error.
    IoError(io::Error),
    /// This enum value represents an UTF8 parsing error
    Utf8Error(FromUtf8Error),
}

/// This enum represents all possible errors when writing a plugin package.
#[derive(Debug)]
pub enum PluginWriteError {
    /// This error occurs if the metadata isn't valid.
    InvalidMeta,
    /// This error occurs if the file name is not valid utf8.
    InvalidFileName,
    /// This enum value represents an IO error.
    IoError(io::Error),
}

impl fmt::Display for PluginReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PluginReadError::InvalidMagicNumber => {
                write!(f, "{}", "Invalid magic number (wrong file?)")
            }
            PluginReadError::NotSupportedFileVersion => {
                write!(f, "{}", "Not supported file version")
            }
            PluginReadError::CorruptedFile => write!(f, "{}", "Corrupted file"),
            PluginReadError::IoError(io_err) => write!(f, "{}", io_err),
            PluginReadError::Utf8Error(utf_err) => write!(f, "{}", utf_err),
        }
    }
}

impl From<io::Error> for PluginReadError {
    #[inline]
    fn from(err: io::Error) -> Self {
        PluginReadError::IoError(err)
    }
}

impl From<FromUtf8Error> for PluginReadError {
    #[inline]
    fn from(err: FromUtf8Error) -> Self {
        PluginReadError::Utf8Error(err)
    }
}

impl fmt::Display for PluginWriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PluginWriteError::InvalidMeta => write!(f, "{}", "Invalid meta data supplied"),
            PluginWriteError::InvalidFileName => write!(f, "{}", "Invalid file name supplied"),
            PluginWriteError::IoError(io_err) => write!(f, "{}", io_err),
        }
    }
}

impl From<io::Error> for PluginWriteError {
    #[inline]
    fn from(err: io::Error) -> Self {
        PluginWriteError::IoError(err)
    }
}
