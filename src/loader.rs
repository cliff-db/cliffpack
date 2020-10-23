use super::error::PluginReadError;
use super::reader::PluginReader;
use super::PluginMeta;
use std::io::{BufReader, Read};

pub enum PluginToken {
    Meta(PluginMeta),
    Code(CodeContext),
}

pub struct CodeContext {
    pub name: String,
    pub content: Vec<u8>,
}

pub struct PluginLoader<R: Read> {
    read: PluginReader<BufReader<R>>,
    meta_read: bool,
}

impl<R: Read> PluginLoader<R> {
    pub fn new(r: PluginReader<BufReader<R>>) -> Result<PluginLoader<R>, PluginReadError> {
        let mut read: PluginReader<BufReader<R>> = r;

        read.verify_magic_number()?;
        read.verify_file_version()?;

        Ok(PluginLoader {
            read,
            meta_read: false,
        })
    }
}

impl<R: Read> Iterator for PluginLoader<R> {
    type Item = Result<PluginToken, PluginReadError>;

    fn next(&mut self) -> Option<Result<PluginToken, PluginReadError>> {
        if self.meta_read {
            if self.read.has_next() {
                match self.read.read_file() {
                    Ok(tuple) => {
                        // Map tuple to struct
                        let ctx = CodeContext {
                            name: tuple.0,
                            content: tuple.1,
                        };

                        Some(Ok(PluginToken::Code(ctx)))
                    }
                    Err(err) => Some(Err(err)),
                }
            } else {
                None
            }
        } else if let Ok(meta) = self.read.read_meta() {
            Some(Ok(PluginToken::Meta(meta)))
        } else {
            Some(Err(PluginReadError::CorruptedFile))
        }
    }
}
