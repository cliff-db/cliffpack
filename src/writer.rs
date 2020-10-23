use super::PluginMeta;
use std::io::{Result, Write};

pub struct PluginWriter<W: Write> {
    pub write: W,
}

impl<W: Write> PluginWriter<W> {
    pub fn write_magic_number(&mut self) -> Result<()> {
        let bytes: [u8; 4] = unsafe { std::mem::transmute(crate::MAGIC_NUMBER.to_be()) };
        self.write.write_all(&bytes)?;

        Ok(())
    }

    #[inline]
    pub fn write_file_version(&mut self) -> Result<()> {
        self.write.write_all(&[crate::FILE_VERSION])
    }

    pub fn write_meta<'a>(&mut self, meta: PluginMeta) -> Result<()> {
        let terminator = b'\0';
        self.write.write_all(meta.name.as_bytes())?;
        self.write.write_all(&[terminator])?;
        self.write.write_all(meta.version.as_bytes())?;
        self.write.write_all(&[terminator])?;
        self.write.write_all(meta.entrypoint.as_bytes())?;
        self.write.write_all(&[terminator])?;

        Ok(())
    }

    pub fn write_file(&mut self, file_name: &str, content: &[u8]) -> Result<()> {
        let name_len = file_name.len() as u16;
        let name_len_bytes: [u8; 2] = unsafe { std::mem::transmute(name_len.to_le()) };
        self.write.write_all(&name_len_bytes)?;
        self.write.write_all(file_name.as_bytes())?;

        let content_len = content.len() as u64;
        let content_len_bytes: [u8; 8] = unsafe { std::mem::transmute(content_len.to_le()) };
        self.write.write_all(&content_len_bytes)?;
        self.write.write_all(content)?;
        Ok(())
    }

    #[inline]
    pub fn flush(&mut self) -> Result<()> {
        self.write.flush()
    }
}
