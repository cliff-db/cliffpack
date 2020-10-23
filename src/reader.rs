use std::io::{BufRead, BufReader, Read};

use super::error::PluginReadError;
use super::PluginMeta;

pub struct PluginReader<R: Read> {
    read: BufReader<R>,
}

impl<R: Read> PluginReader<R> {
    pub fn verify_magic_number(&mut self) -> Result<(), PluginReadError> {
        let magic_bytes: [u8; 4] = unsafe { std::mem::transmute(crate::MAGIC_NUMBER.to_be()) };
        let mut buf: [u8; 4] = [0; 4];
        self.read.read_exact(&mut buf)?;

        if buf == magic_bytes {
            Ok(())
        } else {
            Err(PluginReadError::InvalidMagicNumber)
        }
    }

    pub fn verify_file_version(&mut self) -> Result<(), PluginReadError> {
        let mut buf: [u8; 1] = [0; 1];
        self.read.read_exact(&mut buf)?;

        if buf[0] == crate::FILE_VERSION {
            Ok(())
        } else {
            Err(PluginReadError::NotSupportedFileVersion)
        }
    }

    pub fn read_meta(&mut self) -> Result<PluginMeta, PluginReadError> {
        let terminator = b'\0';

        let mut name_buf = Vec::new();
        self.read.read_until(terminator, &mut name_buf)?;
        let name = unsafe { String::from_utf8_unchecked(name_buf) };

        let mut version_buf = Vec::new();
        self.read.read_until(terminator, &mut version_buf)?;
        let version = unsafe { String::from_utf8_unchecked(version_buf) };

        let mut entrypoint_buf = Vec::new();
        self.read.read_until(terminator, &mut entrypoint_buf)?;
        let entrypoint = unsafe { String::from_utf8_unchecked(entrypoint_buf) };

        let meta = PluginMeta {
            name,
            version,
            entrypoint,
        };

        Ok(meta)
    }

    pub fn read_file(&mut self) -> Result<(String, Vec<u8>), PluginReadError> {
        let mut name_len_buf: [u8; 2] = [0; 2];
        self.read.read_exact(&mut name_len_buf)?;
        let name_len_bits: u16 = unsafe { std::mem::transmute(u16::from_le_bytes(name_len_buf)) };
        let name_len = name_len_bits as usize;

        let mut name_buf = Vec::with_capacity(name_len);
        self.read.read_exact(&mut name_buf)?;

        let mut content_len_buf: [u8; 8] = [0; 8];
        self.read.read_exact(&mut content_len_buf)?;
        let content_len_bits: u64 =
            unsafe { std::mem::transmute(u64::from_le_bytes(content_len_buf)) };
        let content_len = content_len_bits as usize;

        let mut content_buf = Vec::with_capacity(content_len);
        self.read.read_exact(&mut content_buf)?;

        let name = String::from_utf8(name_buf)?;
        Ok((name, content_buf))
    }

    #[inline]
    pub fn has_next(&self) -> bool {
        // FIXME: Here is a ownership conflict
        // self.read.bytes().peekable().peek().is_some()
        unimplemented!()
    }
}
