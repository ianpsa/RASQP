//! Linux filtering of locally authored experiments, not a filesystem read jail.
use crate::{Error, Result};
#[cfg(target_os = "linux")]
pub fn restrict() -> Result<()> {
    use seccompiler::{
        BpfProgram, SeccompAction, SeccompCmpArgLen, SeccompCmpOp, SeccompCondition, SeccompFilter,
        SeccompRule,
    };
    use std::collections::BTreeMap;
    let mut rules = BTreeMap::new();
    for syscall in [
        libc::SYS_socket,
        libc::SYS_connect,
        libc::SYS_bind,
        libc::SYS_listen,
        libc::SYS_accept4,
        libc::SYS_execve,
        libc::SYS_execveat,
        libc::SYS_ptrace,
        libc::SYS_openat2,
        libc::SYS_unlinkat,
        libc::SYS_renameat,
        libc::SYS_renameat2,
        libc::SYS_mkdirat,
        libc::SYS_linkat,
        libc::SYS_symlinkat,
        libc::SYS_truncate,
        libc::SYS_fchmod,
        libc::SYS_fchmodat,
        libc::SYS_fchown,
        libc::SYS_mknodat,
        libc::SYS_mount,
        libc::SYS_io_uring_setup,
    ] {
        rules.insert(syscall, vec![]);
    }
    #[cfg(target_arch = "x86_64")]
    for syscall in [
        libc::SYS_creat,
        libc::SYS_unlink,
        libc::SYS_rename,
        libc::SYS_mkdir,
        libc::SYS_rmdir,
        libc::SYS_link,
        libc::SYS_symlink,
        libc::SYS_chmod,
        libc::SYS_chown,
        libc::SYS_mknod,
    ] {
        rules.insert(syscall, vec![]);
    }
    let mut opens = vec![(libc::SYS_openat, 2)];
    #[cfg(target_arch = "x86_64")]
    opens.push((libc::SYS_open, 1));
    for (syscall, arg) in opens {
        let mut conditions = vec![];
        for flag in [
            libc::O_WRONLY,
            libc::O_RDWR,
            libc::O_CREAT,
            libc::O_TRUNC,
            libc::O_APPEND,
        ] {
            let condition = SeccompCondition::new(
                arg,
                SeccompCmpArgLen::Dword,
                SeccompCmpOp::MaskedEq(flag as u64),
                flag as u64,
            )
            .map_err(|e| Error::Operation(e.to_string()))?;
            conditions.push(
                SeccompRule::new(vec![condition]).map_err(|e| Error::Operation(e.to_string()))?,
            );
        }
        rules.insert(syscall, conditions);
    }
    let filter: BpfProgram = SeccompFilter::new(
        rules,
        SeccompAction::Allow,
        SeccompAction::Errno(libc::EPERM as u32),
        std::env::consts::ARCH
            .try_into()
            .map_err(|e: seccompiler::BackendError| Error::Operation(e.to_string()))?,
    )
    .map_err(|e| Error::Operation(e.to_string()))?
    .try_into()
    .map_err(|e: seccompiler::BackendError| Error::Operation(e.to_string()))?;
    seccompiler::apply_filter_all_threads(&filter).map_err(|e| Error::Operation(e.to_string()))?;
    if std::net::TcpListener::bind("127.0.0.1:0").is_ok()
        || std::fs::File::create("rasqp-sandbox-must-not-write").is_ok()
    {
        return Err(Error::Invalid("sandbox self test"));
    }
    Ok(())
}
#[cfg(not(target_os = "linux"))]
pub fn restrict() -> Result<()> {
    Err(Error::Invalid(
        "seccomp research runner requires Linux; use explicitly unconfined mode on other platforms",
    ))
}
