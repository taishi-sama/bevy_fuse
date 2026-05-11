use bevy::log::{trace, warn};
use fuser::{AccessFlags, BsdFileFlags, CopyFileRangeFlags, Errno, Filesystem, FopenFlags, INodeNo, IoctlFlags, LockOwner, OpenFlags, PollEvents, PollFlags, PollNotifier, RenameFlags, WriteFlags};

use crate::fuser::{FusECS, types::*};

impl Filesystem for FusECS {
    fn init(
        &mut self,
        _req: &fuser::Request,
        _config: &mut fuser::KernelConfig,
    ) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn destroy(&mut self) {}

    fn lookup(
        &self,
        _req: &fuser::Request,
        parent: INodeNo,
        name: &std::ffi::OsStr,
        reply: fuser::ReplyEntry,
    ) {
        trace!("[Implemented] lookup(parent: {parent:#x?}, name {name:?})");
        // reply.error(ENOSYS);
        self.lookup
            .send((
                LookupParams {
                    parent,
                    name: name.into(),
                },
                reply,
            ))
            .unwrap();
    }

    fn forget(&self, _req: &fuser::Request, _ino: INodeNo, _nlookup: u64) {}

    fn getattr(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        fh: Option<fuser::FileHandle>,
        reply: fuser::ReplyAttr,
    ) {
        trace!(
            "[Implemented] getattr(ino: {ino:#x?}, fh: {fh:#x?}, unique: {:#?})",
            _req.unique()
        );
        self.getattr.send((GetAttrParams { ino }, reply)).unwrap();
    }

    fn setattr(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<fuser::TimeOrNow>,
        mtime: Option<fuser::TimeOrNow>,
        ctime: Option<std::time::SystemTime>,
        fh: Option<fuser::FileHandle>,
        crtime: Option<std::time::SystemTime>,
        chgtime: Option<std::time::SystemTime>,
        bkuptime: Option<std::time::SystemTime>,
        flags: Option<BsdFileFlags>,
        reply: fuser::ReplyAttr,
    ) {
        trace!(
            "[Implemented] setattr(ino: {ino:#x?}, mode: {mode:?}, uid: {uid:?}, \
            gid: {gid:?}, size: {size:?}, fh: {fh:?}, flags: {flags:?})"
        );
        self.setattr
            .send((
                SetattrParams {
                    ino,
                    mode,
                    uid,
                    gid,
                    size,
                    atime,
                    mtime,
                    ctime,
                    fh,
                    crtime,
                    chgtime,
                    bkuptime,
                },
                reply,
            ))
            .unwrap();
        // reply.error(Errno::ENOSYS);
    }

    fn readlink(&self, _req: &fuser::Request, ino: INodeNo, reply: fuser::ReplyData) {
        warn!("[Not Implemented] readlink(ino: {ino:#x?})");
        reply.error(Errno::ENOSYS);
    }

    fn mknod(
        &self,
        _req: &fuser::Request,
        parent: INodeNo,
        name: &std::ffi::OsStr,
        mode: u32,
        umask: u32,
        rdev: u32,
        reply: fuser::ReplyEntry,
    ) {
        trace!(
            "[Implemented] mknod(parent: {parent:#x?}, name: {name:?}, \
            mode: {mode}, umask: {umask:#x?}, rdev: {rdev})"
        );
        self.mknod
            .send((
                MknodParams {
                    parent,
                    name: name.to_owned(),
                    mode,
                    rdev,
                },
                reply,
            ))
            .unwrap();
        // reply.error(Errno::ENOSYS);
    }

    fn mkdir(
        &self,
        _req: &fuser::Request,
        parent: INodeNo,
        name: &std::ffi::OsStr,
        mode: u32,
        umask: u32,
        reply: fuser::ReplyEntry,
    ) {
        trace!(
            "[Implemented] mkdir(parent: {parent:#x?}, name: {name:?}, mode: {mode}, umask: {umask:#x?})"
        );
        self.mkdir
            .send((
                MkdirParams {
                    parent,
                    name: name.to_owned(),
                    mode,
                },
                reply,
            ))
            .unwrap();
        // reply.error(Errno::ENOSYS);
    }

    fn unlink(
        &self,
        _req: &fuser::Request,
        parent: INodeNo,
        name: &std::ffi::OsStr,
        reply: fuser::ReplyEmpty,
    ) {
        warn!("[Not Implemented] unlink(parent: {parent:#x?}, name: {name:?})",);
        reply.error(Errno::ENOSYS);
    }

    fn rmdir(
        &self,
        _req: &fuser::Request,
        parent: INodeNo,
        name: &std::ffi::OsStr,
        reply: fuser::ReplyEmpty,
    ) {
        warn!("[Not Implemented] rmdir(parent: {parent:#x?}, name: {name:?})",);
        reply.error(Errno::ENOSYS);
    }

    fn symlink(
        &self,
        _req: &fuser::Request,
        parent: INodeNo,
        link_name: &std::ffi::OsStr,
        target: &std::path::Path,
        reply: fuser::ReplyEntry,
    ) {
        warn!(
            "[Not Implemented] symlink(parent: {parent:#x?}, link_name: {link_name:?}, target: {target:?})",
        );
        reply.error(Errno::EPERM);
    }

    fn rename(
        &self,
        _req: &fuser::Request,
        parent: INodeNo,
        name: &std::ffi::OsStr,
        newparent: INodeNo,
        newname: &std::ffi::OsStr,
        flags: RenameFlags,
        reply: fuser::ReplyEmpty,
    ) {
        warn!(
            "[Not Implemented] rename(parent: {parent:#x?}, name: {name:?}, \
            newparent: {newparent:#x?}, newname: {newname:?}, flags: {flags})",
        );
        reply.error(Errno::ENOSYS);
    }

    fn link(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        newparent: INodeNo,
        newname: &std::ffi::OsStr,
        reply: fuser::ReplyEntry,
    ) {
        warn!(
            "[Not Implemented] link(ino: {ino:#x?}, newparent: {newparent:#x?}, newname: {newname:?})"
        );
        reply.error(Errno::EPERM);
    }

    fn open(&self, _selfeq: &fuser::Request, ino: INodeNo, flags: OpenFlags, reply: fuser::ReplyOpen) {
        self.open.send((OpenParams { ino, flags }, reply)).unwrap();
        // reply.opened(0, 0);
    }

    fn read(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        offset: u64,
        size: u32,
        flags: OpenFlags,
        lock_owner: Option<LockOwner>,
        reply: fuser::ReplyData,
    ) {
        trace!(
            "[Implemented] read(ino: {ino:#x?}, fh: {fh}, offset: {offset}, \
            size: {size}, flags: {flags:#x?}, lock_owner: {lock_owner:?})"
        );
        self.read
            .send((
                ReadParams {
                    ino,
                    fh,
                    offset,
                    size,
                    flags,
                },
                reply,
            ))
            .unwrap();
        // reply.error(Errno::ENOSYS);
    }

    fn write(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        offset: u64,
        data: &[u8],
        write_flags: WriteFlags,
        flags: OpenFlags,
        lock_owner: Option<LockOwner>,
        reply: fuser::ReplyWrite,
    ) {
        trace!(
            "[Implemented] write(ino: {ino:#x?}, fh: {fh}, offset: {offset}, \
            data.len(): {}, write_flags: {write_flags:#x?}, flags: {flags:#x?}, \
            lock_owner: {lock_owner:?})",
            data.len()
        );
        self.write
            .send((
                WriteParams {
                    ino,
                    fh,
                    offset,
                    data: data.to_vec(),
                    flags,
                },
                reply,
            ))
            .unwrap();
        // reply.error(Errno::ENOSYS);
    }

    fn flush(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        lock_owner: LockOwner,
        reply: fuser::ReplyEmpty,
    ) {
        trace!("[Implemented] flush(ino: {ino:#x?}, fh: {fh}, lock_owner: {lock_owner:?})");
        // reply.error(Errno::ENOSYS);
        reply.ok();
    }

    fn release(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        _flags: OpenFlags,
        _lock_owner: Option<LockOwner>,
        _flush: bool,
        reply: fuser::ReplyEmpty,
    ) {
        self.release
            .send((ReleaseParams { ino, fh }, reply))
            .unwrap();
        // reply.ok();
    }

    fn fsync(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        datasync: bool,
        reply: fuser::ReplyEmpty,
    ) {
        trace!("[Implemented] fsync(ino: {ino:#x?}, fh: {fh}, datasync: {datasync})");
        // reply.error(Errno::ENOSYS);
        reply.ok();
    }

    fn opendir(
        &self,
        _req: &fuser::Request,
        _ino: INodeNo,
        _flags: OpenFlags,
        reply: fuser::ReplyOpen,
    ) {
        reply.opened(fuser::FileHandle(0), FopenFlags::empty());
    }

    fn readdir(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        offset: u64,
        reply: fuser::ReplyDirectory,
    ) {
        trace!("[Implemented] readdir(ino: {ino:#x?}, fh: {fh}, offset: {offset})");
        self.readdir
            .send((ReaddirParams { ino, offset }, reply))
            .unwrap();

        // reply.error(Errno::ENOSYS);
    }

    fn readdirplus(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        offset: u64,
        reply: fuser::ReplyDirectoryPlus,
    ) {
        warn!("[Not Implemented] readdirplus(ino: {ino:#x?}, fh: {fh}, offset: {offset})");
        reply.error(Errno::ENOSYS);
    }

    fn releasedir(
        &self,
        _req: &fuser::Request,
        _ino: INodeNo,
        _fh: fuser::FileHandle,
        _flags: OpenFlags,
        reply: fuser::ReplyEmpty,
    ) {
        reply.ok();
    }

    fn fsyncdir(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        datasync: bool,
        reply: fuser::ReplyEmpty,
    ) {
        warn!("[Not Implemented] fsyncdir(ino: {ino:#x?}, fh: {fh}, datasync: {datasync})");
        reply.error(Errno::ENOSYS);
    }

    fn statfs(&self, _req: &fuser::Request, _ino: INodeNo, reply: fuser::ReplyStatfs) {
        trace!("[Implemented] statfs");

        self.statfs.send(((), reply)).unwrap();
        // reply.statfs(0, 0, 0, 1, 0, 512, 255, 0);
    }

    fn setxattr(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        name: &std::ffi::OsStr,
        _value: &[u8],
        flags: i32,
        position: u32,
        reply: fuser::ReplyEmpty,
    ) {
        warn!(
            "[Not Implemented] setxattr(ino: {ino:#x?}, name: {name:?}, \
            flags: {flags:#x?}, position: {position})"
        );
        reply.error(Errno::ENOSYS);
    }

    fn getxattr(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        name: &std::ffi::OsStr,
        size: u32,
        reply: fuser::ReplyXattr,
    ) {
        warn!("[Not Implemented] getxattr(ino: {ino:#x?}, name: {name:?}, size: {size})");
        reply.error(Errno::ENOSYS);
    }

    fn listxattr(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        size: u32,
        reply: fuser::ReplyXattr,
    ) {
        warn!("[Not Implemented] listxattr(ino: {ino:#x?}, size: {size})");
        reply.error(Errno::ENOSYS);
    }

    fn removexattr(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        name: &std::ffi::OsStr,
        reply: fuser::ReplyEmpty,
    ) {
        warn!("[Not Implemented] removexattr(ino: {ino:#x?}, name: {name:?})");
        reply.error(Errno::ENOSYS);
    }

    fn access(&self, _req: &fuser::Request, ino: INodeNo, mask: AccessFlags, reply: fuser::ReplyEmpty) {
        warn!("[Not Implemented] access(ino: {ino:#x?}, mask: {mask})");
        reply.error(Errno::ENOSYS);
    }

    fn create(
        &self,
        _req: &fuser::Request,
        parent: INodeNo,
        name: &std::ffi::OsStr,
        mode: u32,
        umask: u32,
        flags: i32,
        reply: fuser::ReplyCreate,
    ) {
        warn!(
            "[Not Implemented] create(parent: {parent:#x?}, name: {name:?}, mode: {mode}, \
            umask: {umask:#x?}, flags: {flags:#x?})"
        );
        reply.error(Errno::ENOSYS);
    }

    fn getlk(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        lock_owner: LockOwner,
        start: u64,
        end: u64,
        typ: i32,
        pid: u32,
        reply: fuser::ReplyLock,
    ) {
        warn!(
            "[Not Implemented] getlk(ino: {ino:#x?}, fh: {fh}, lock_owner: {lock_owner}, \
            start: {start}, end: {end}, typ: {typ}, pid: {pid})"
        );
        reply.error(Errno::ENOSYS);
    }

    fn setlk(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        lock_owner: LockOwner,
        start: u64,
        end: u64,
        typ: i32,
        pid: u32,
        sleep: bool,
        reply: fuser::ReplyEmpty,
    ) {
        warn!(
            "[Not Implemented] setlk(ino: {ino:#x?}, fh: {fh}, lock_owner: {lock_owner}, \
            start: {start}, end: {end}, typ: {typ}, pid: {pid}, sleep: {sleep})"
        );
        reply.error(Errno::ENOSYS);
    }

    fn bmap(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        blocksize: u32,
        idx: u64,
        reply: fuser::ReplyBmap,
    ) {
        warn!("[Not Implemented] bmap(ino: {ino:#x?}, blocksize: {blocksize}, idx: {idx})",);
        reply.error(Errno::ENOSYS);
    }

    fn ioctl(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        flags: IoctlFlags,
        cmd: u32,
        in_data: &[u8],
        out_size: u32,
        reply: fuser::ReplyIoctl,
    ) {
        warn!(
            "[Not Implemented] ioctl(ino: {ino:#x?}, fh: {fh}, flags: {flags}, \
            cmd: {cmd}, in_data.len(): {}, out_size: {out_size})",
            in_data.len()
        );
        reply.error(Errno::ENOSYS);
    }

    fn poll(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        ph: PollNotifier,
        events: PollEvents,
        flags: PollFlags,
        reply: fuser::ReplyPoll,
    ) {
        warn!(
            "[Not Implemented] poll(ino: {ino:#x?}, fh: {fh}, \
            ph: {ph:?}, events: {events}, flags: {flags})"
        );
        reply.error(Errno::ENOSYS);
    }

    fn fallocate(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        offset: u64,
        length: u64,
        mode: i32,
        reply: fuser::ReplyEmpty,
    ) {
        warn!(
            "[Not Implemented] fallocate(ino: {ino:#x?}, fh: {fh}, \
            offset: {offset}, length: {length}, mode: {mode})"
        );
        reply.error(Errno::ENOSYS);
    }

    fn lseek(
        &self,
        _req: &fuser::Request,
        ino: INodeNo,
        fh: fuser::FileHandle,
        offset: i64,
        whence: i32,
        reply: fuser::ReplyLseek,
    ) {
        warn!(
            "[Not Implemented] lseek(ino: {ino:#x?}, fh: {fh}, \
            offset: {offset}, whence: {whence})"
        );
        reply.error(Errno::ENOSYS);
    }

    fn copy_file_range(
        &self,
        _req: &fuser::Request,
        ino_in: INodeNo,
        fh_in: fuser::FileHandle,
        offset_in: u64,
        ino_out: INodeNo,
        fh_out: fuser::FileHandle,
        offset_out: u64,
        len: u64,
        flags: CopyFileRangeFlags,
        reply: fuser::ReplyWrite,
    ) {
        warn!(
            "[Not Implemented] copy_file_range(ino_in: {ino_in:#x?}, fh_in: {fh_in}, \
            offset_in: {offset_in}, ino_out: {ino_out:#x?}, fh_out: {fh_out}, \
            offset_out: {offset_out}, len: {len}, flags: {flags:?})"
        );
        reply.error(Errno::ENOSYS);
    }
}
