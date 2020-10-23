use super::error::PluginWriteError;
use super::writer;
use super::PluginMeta;
use std::fs::File;
use std::io::BufWriter;
use std::path::{Path, PathBuf};

pub struct PluginCreator {
    meta: PluginMeta,
    files: Vec<PathBuf>,
}

impl PluginCreator {
    pub fn new(meta: PluginMeta) -> PluginCreator {
        PluginCreator {
            meta,
            files: Vec::new(),
        }
    }

    #[inline]
    pub fn add_file(&mut self, file: PathBuf) {
        self.files.push(file);
    }

    pub fn create_plugin<P: AsRef<Path>>(&mut self, output: P) -> Result<(), PluginWriteError> {
        let file = File::open(output)?;

        // BufWriter is used to reduce system-write-calls and this improves performance
        let write = BufWriter::new(file);

        let mut writer = writer::PluginWriter { write };

        writer.write_magic_number()?;
        writer.write_file_version()?;
        writer.write_meta(self.meta.clone())?;
        writer.flush()?;

        for path in self.files.iter() {
            let path_buf: &PathBuf = path;

            // read file to memory (I hope nobody creates code files larger than the maximum memory size)
            let buf = std::fs::read(path_buf)?;
            match path_buf.as_os_str().to_str() {
                Some(path_str) => writer.write_file(path_str, &buf)?,
                None => {
                    // sorry, for that explicit return
                    return Err(PluginWriteError::InvalidFileName);
                }
            };
            writer.flush()?;
        }

        Ok(())
    }
}
