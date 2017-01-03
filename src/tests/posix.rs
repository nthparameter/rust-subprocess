extern crate tempdir;

use super::super::{Popen, PopenConfig, ExitStatus, Redirection};
use std::fs::File;
use std::io::Write;
use std::mem;
use super::super::posix;

use self::tempdir::TempDir;

use tests::common::read_whole_file;

#[test]
fn err_terminate() {
    let mut p = Popen::create(&["sleep", "5"], PopenConfig::default()).unwrap();
    assert!(p.poll().is_none());
    p.terminate().unwrap();
    assert!(p.wait().unwrap() == ExitStatus::Signaled(posix::SIGTERM as u8));
}

#[test]
fn write_to_subprocess() {
    let tmpdir = TempDir::new("test").unwrap();
    let tmpname = tmpdir.path().join("output");
    let mut p = Popen::create(
        &["dd".to_string(), format!("of={}", tmpname.display()), "status=none".to_string()],
        PopenConfig {
            stdin: Redirection::Pipe,
            ..Default::default()
        })
        .unwrap();
    p.stdin.as_mut().unwrap().write_all(b"foo").unwrap();
    mem::drop(p.stdin.take());
    assert!(p.wait().unwrap() == ExitStatus::Exited(0));
    assert!(read_whole_file(File::open(tmpname).unwrap()) == "foo");
}

#[test]
fn waitpid_echild() {
    let mut p = Popen::create(&["true"], PopenConfig::default())
        .unwrap();
    let pid = p.pid().unwrap();
    let (wpid, status) = posix::waitpid(pid, 0).unwrap();
    assert!(wpid == pid);
    assert!(status == ExitStatus::Exited(0));
    assert!(p.wait().unwrap() == ExitStatus::Undetermined);
}
