use std::path::PathBuf;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref MANIFEST: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    pub static ref WORKSPACE: PathBuf = MANIFEST.join("..").join("..");
    pub static ref TARGET: PathBuf = WORKSPACE.join("target");
    pub static ref CPP: PathBuf = WORKSPACE.join("cpp");
    pub static ref LIBSTATIC: PathBuf = CPP.join("libstatic");
    pub static ref INCLUDE: PathBuf = CPP.join("src").join("main").join("include");
    pub static ref FFI_INCLUDE: PathBuf = INCLUDE.join("ffi");
}
