use cc;

mod lkm {
    use std::process::{Command, Stdio};
    pub fn gen_driver_source() {
        let out_dir = std::env::var_os("OUT_DIR").unwrap();
        let kmod_source = &std::path::Path::new(&out_dir)
            .join("kmod_source.zip")
            .display()
            .to_string();

        let _package_source: std::process::Output = Command::new("zip")
            .args(&[kmod_source, "-r", "LKM"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
    }
    pub fn gen_driver_kmod_info() {
        let git_result: std::process::Output = Command::new("git")
            .args(&["log", "--pretty=tformat:\"%H\"", "-n1", "."])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        let commit_id = std::str::from_utf8(&git_result.stdout)
            .unwrap()
            .trim()
            .trim_matches('"');

        let kmod_name_result: std::process::Output = Command::new("sh")
            .args(&[
                "-c",
                "grep \"MODULE_NAME\" LKM/Makefile | grep -m 1 \":=\" | awk '{print $3}'",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        let kmod_name = std::str::from_utf8(&kmod_name_result.stdout)
            .unwrap()
            .trim()
            .trim_matches('"');

        let kmod_version_result: std::process::Output = Command::new("sh")
            .args(&[
                "-c",
                "cat LKM/include/kprobe.h | grep SMITH_VERSION | awk -F '\"' '{print $2}'",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        let kmod_version = std::str::from_utf8(&kmod_version_result.stdout)
            .unwrap()
            .trim()
            .trim_matches('"');

        let out_dir = std::env::var_os("OUT_DIR").unwrap();
        let path = std::path::Path::new(&out_dir).join("kmod_info.rs");
        std::fs::write(
            &path,
            format!(
                "pub const kmod_name: &str = \"{}\";pub const kmod_version: &str = \"{}\";pub const kmod_commit: &str = \"{}\";",
                kmod_name,
                kmod_version,
                commit_id
            ),
        )
        .unwrap();
    }
}


fn main() {
    println!("cargo:rerun-if-changed={}", "LKM/include");
    println!("cargo:rerun-if-changed={}", "LKM/zua");
    lkm::gen_driver_kmod_info();
    lkm::gen_driver_source();

    cc::Build::new()
        .include("LKM/include")
        .include("LKM/zua")
        .file("LKM/test/xfer.c")
        .file("LKM/test/ring.c")
        .file("LKM/zua/zua_scanner.c")
        .file("LKM/zua/zua_parser.c")
        .file("LKM/zua/zua_type.c")
        .compile("ring");
}
