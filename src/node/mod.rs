pub mod directory;
pub mod file;
pub mod filehandle;

use std::time::SystemTime;

use bevy::prelude::*;
use fuser::{FileAttr, FileType};

use crate::{fuser::Inode, node::file::File};

#[derive(Debug, Component, Clone)]
pub struct INode {
    pub atime: SystemTime, // 1970-01-01 00:00:00
    pub mtime: SystemTime,
    pub ctime: SystemTime,
    pub crtime: SystemTime,
    pub kind: FileType,
    pub perm: u16,
    pub nlink: u32,
    pub uid: u32,
    pub gid: u32,
    pub rdev: u32,
    pub flags: u32,
    pub blksize: u32,
    pub open_file_handles: u64,
}
impl INode {
    pub fn new_file() -> Self {
        let time = SystemTime::now();
        Self {
            atime: time,
            mtime: time,
            ctime: time,
            crtime: time,
            kind: FileType::RegularFile,
            perm: 0o755,
            nlink: 1,
            uid: 501,
            gid: 20,
            rdev: 0,
            flags: 0,
            blksize: 512,
            open_file_handles: 0,
        }
    }
    pub fn new_directory() -> Self {
        let time = SystemTime::now();
        Self {
            atime: time,
            mtime: time,
            ctime: time,
            crtime: time,
            kind: FileType::Directory,
            perm: 0o755,
            nlink: 2,
            uid: 501,
            gid: 20,
            rdev: 0,
            flags: 0,
            blksize: 512,
            open_file_handles: 0,
        }
    }
    pub fn get_file_attrb_obj(&self, ino: Inode, file: Option<&File>) -> FileAttr {
        self.get_file_attrb(ino, file.map(|x| x.get_size()).unwrap_or(0))
    }
    pub fn get_file_attrb_obj_mut(&self, ino: Inode, file: Option<Mut<File>>) -> FileAttr {
        self.get_file_attrb(ino, file.map(|x| x.get_size()).unwrap_or(0))
    }
    pub fn get_file_attrb(&self, ino: Inode, size: u64) -> FileAttr {
        FileAttr {
            ino,
            size,
            blocks: size.div_ceil(self.blksize as u64),
            atime: self.atime,
            mtime: self.mtime,
            ctime: self.ctime,
            crtime: self.crtime,
            kind: self.kind,
            perm: self.perm,
            nlink: self.nlink,
            uid: self.uid,
            gid: self.gid,
            rdev: self.rdev,
            blksize: self.blksize,
            flags: self.flags,
        }
    }
}
