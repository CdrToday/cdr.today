//! Ah, This is the IPC part
use crate::{Error, Query, Result};

/// IPC using memory
///
/// This can be implemented a future
pub struct Ipc {
    /// Stack that stores queries
    pub stack: Vec<Query>,
    /// Just use string as result, 0 is id, 1 is result
    pub result: Vec<String>,
}

impl Ipc {
    /// Read a query
    pub fn read(&self) -> Result<&Query> {
        Ok(&self.stack[self.stack.len()])
    }

    /// Write a query
    pub fn write(&mut self, resp: String) -> Result<()> {
        self.result.push(resp);
        Ok(())
    }

    /// Input a query, for client
    pub fn input(&mut self, q: Query) -> Result<usize> {
        self.stack.push(q);
        Ok(self.stack.len())
    }

    /// Output a result, for client
    pub fn output(&mut self, id: usize) -> Result<&String> {
        if self.result.len() > id {
            Ok(&self.result[id])
        } else {
            Err(Error::SegmentationFault)
        }
    }
}
