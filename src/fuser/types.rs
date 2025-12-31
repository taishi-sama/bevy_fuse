use std::{ffi::OsString, time::SystemTime};

use fuser::TimeOrNow;

use crate::fuser::{Fh, Inode};

#[derive(Debug)]
pub struct GetAttrParams {
    pub ino: Inode,
}

#[derive(Debug)]
pub struct LookupParams {
    pub parent: Inode,
    pub name: OsString,
}

#[derive(Debug)]
pub struct ReaddirParams {
    pub ino: Inode,
    pub offset: i64,
}

#[derive(Debug)]
pub struct OpenParams {
    pub ino: Inode,
    pub flags: i32,
}

#[derive(Debug)]
pub struct ReleaseParams {
    pub ino: Inode,
    pub fh: Fh,
}

#[derive(Debug)]
pub struct ReadParams {
    pub ino: Inode,
    pub fh: Fh,
    pub offset: i64,
    pub size: u32,
    pub flags: i32,
}

#[derive(Debug)]
pub struct WriteParams {
    pub ino: Inode,
    pub fh: Fh,
    pub offset: i64,
    pub data: Vec<u8>,
    pub flags: i32,
}

#[derive(Debug)]
pub struct SetattrParams {
    pub ino: Inode,
    pub mode: Option<u32>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
    pub size: Option<u64>,
    pub atime: Option<TimeOrNow>,
    pub mtime: Option<TimeOrNow>,
    pub ctime: Option<SystemTime>,
    pub fh: Option<u64>,
    pub crtime: Option<SystemTime>,
    pub chgtime: Option<SystemTime>,
    pub bkuptime: Option<SystemTime>,
}

#[derive(Debug)]
pub struct MkdirParams {
    pub parent: Inode,
    pub name: OsString,
    pub mode: u32,
}

#[derive(Debug)]
pub struct MknodParams {
    pub parent: Inode,
    pub name: OsString,
    pub mode: u32,
    pub rdev: u32,
}
