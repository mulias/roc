use crate::target::arch_str;
use std::io;
use std::path::Path;
use std::process::{Child, Command};
use target_lexicon::{Architecture, OperatingSystem, Triple};

pub fn link(
    target: &Triple,
    binary_path: &Path,
    host_input_path: &Path,
    dest_filename: &Path,
) -> io::Result<Child> {
    match target {
        Triple {
            architecture: Architecture::X86_64,
            operating_system: OperatingSystem::Linux,
            ..
        } => link_linux(
            arch_str(target),
            binary_path,
            host_input_path,
            dest_filename,
        ),
        Triple {
            architecture: Architecture::X86_64,
            operating_system: OperatingSystem::Darwin,
            ..
        } => todo!("link macos"),
        _ => panic!("TODO gracefully handle unsupported target: {:?}", target),
    }
}

fn link_linux(
    arch: &str,
    binary_path: &Path,
    host_input_path: &Path,
    dest_filename: &Path,
) -> io::Result<Child> {
    Command::new("ld")
        .args(&[
            "-arch",
            arch,
            "/usr/lib/x86_64-linux-gnu/crti.o",
            "/usr/lib/x86_64-linux-gnu/crtn.o",
            "/usr/lib/x86_64-linux-gnu/Scrt1.o",
            "-dynamic-linker",
            "/lib64/ld-linux-x86-64.so.2",
            // Libraries - see https://github.com/rtfeldman/roc/pull/554#discussion_r496365925
            // for discussion and further references
            "-lc",
            "-lm",
            "-lpthread",
            "-ldl",
            "-lrt",
            "-lutil",
            "-lc_nonshared",
            // "-lc++", // TODO shouldn't we need this?
            // "-lgcc", // TODO will eventually need compiler_rt from gcc or something - see https://github.com/rtfeldman/roc/pull/554#discussion_r496370840
            // "-lunwind", // TODO will eventually need this, see https://github.com/rtfeldman/roc/pull/554#discussion_r496370840
            "-o",
            binary_path.to_str().unwrap(),     // app
            host_input_path.to_str().unwrap(), // host.o
            dest_filename.to_str().unwrap(),   // roc_app.o
        ])
        .spawn()
}
