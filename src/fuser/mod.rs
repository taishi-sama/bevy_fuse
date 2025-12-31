pub mod fus_ecs_impl;
pub mod types;

use std::{
    path::PathBuf,
    sync::mpsc::{Receiver, Sender, channel},
};

use bevy::prelude::*;
use fuser::{
    BackgroundSession, MountOption, ReplyAttr, ReplyData, ReplyDirectory, ReplyEmpty, ReplyEntry,
    ReplyOpen, ReplyStatfs, ReplyWrite,
};

use crate::fuser::types::*;

#[derive(Debug, Resource)]
pub struct FuseParams {
    pub mountpoint: PathBuf,
    pub auto_unmount: bool,
}

#[derive(Debug, Default)]
pub struct FuserState(pub Option<FuserInternal>);

pub type Inode = u64;
pub type Fh = u64;

type Lookup = (LookupParams, ReplyEntry);
type Getattr = (GetAttrParams, ReplyAttr);
type Readdir = (ReaddirParams, ReplyDirectory);
type Statdir = ((), ReplyStatfs);
type Open = (OpenParams, ReplyOpen);
type Release = (ReleaseParams, ReplyEmpty);
type Read = (ReadParams, ReplyData);
type Write = (WriteParams, ReplyWrite);
type Setattr = (SetattrParams, ReplyAttr);
type Mkdir = (MkdirParams, ReplyEntry);
type Mknod = (MknodParams, ReplyEntry);

#[derive(Debug)]
pub struct FuserInternal {
    pub fuser_thread: BackgroundSession,
    pub lookup: Receiver<Lookup>,
    pub getattr: Receiver<Getattr>,
    pub readdir: Receiver<Readdir>,
    pub statfs: Receiver<Statdir>,
    pub open: Receiver<Open>,
    pub release: Receiver<Release>,
    pub read: Receiver<Read>,
    pub write: Receiver<Write>,
    pub setattr: Receiver<Setattr>,
    pub mkdir: Receiver<Mkdir>,
    pub mknod: Receiver<Mknod>,
}
#[derive(Debug)]
pub struct FusECS {
    lookup: Sender<Lookup>,
    getattr: Sender<Getattr>,
    readdir: Sender<Readdir>,
    statfs: Sender<Statdir>,
    open: Sender<Open>,
    release: Sender<Release>,
    read: Sender<Read>,
    write: Sender<Write>,
    setattr: Sender<Setattr>,
    mkdir: Sender<Mkdir>,
    mknod: Sender<Mknod>,
}

pub fn construct_fusecs(params: &FuseParams) -> FuserInternal {
    let mut options = vec![MountOption::FSName("bevy_fuse".to_string())];
    if params.auto_unmount {
        options.push(MountOption::AutoUnmount);
    }
    let lookup_pair = channel();
    let getattr_pair = channel();
    let readdir_pair = channel();
    let statfs_pair = channel();
    let open_pair = channel();
    let release_pair = channel();
    let read_pair = channel();
    let write_pair = channel();
    let setattr_pair = channel();
    let mkdir_pair = channel();
    let mknod_pair = channel();

    let fus_ecs = FusECS {
        lookup: lookup_pair.0,
        getattr: getattr_pair.0,
        readdir: readdir_pair.0,
        statfs: statfs_pair.0,
        open: open_pair.0,
        release: release_pair.0,
        read: read_pair.0,
        write: write_pair.0,
        setattr: setattr_pair.0,
        mkdir: mkdir_pair.0,
        mknod: mknod_pair.0,
    };
    let result = fuser::spawn_mount2(fus_ecs, &params.mountpoint, &options).unwrap();
    let fuser = FuserInternal {
        fuser_thread: result,
        lookup: lookup_pair.1,
        getattr: getattr_pair.1,
        readdir: readdir_pair.1,
        statfs: statfs_pair.1,
        open: open_pair.1,
        release: release_pair.1,
        read: read_pair.1,
        write: write_pair.1,
        setattr: setattr_pair.1,
        mkdir: mkdir_pair.1,
        mknod: mknod_pair.1,
    };
    fuser
}
