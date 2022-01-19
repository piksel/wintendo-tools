


fn main() {
    println!("cargo:rustc-link-arg-bin=shoutmode=/MANIFEST:embed");
    println!("cargo:rustc-link-arg-bin=shoutmode=/MANIFESTUAC:level=\"requireAdministrator\"");
}