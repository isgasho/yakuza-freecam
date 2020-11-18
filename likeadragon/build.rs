extern crate winres;

fn main() {
    let mut res = winres::WindowsResource::new();

    res.set_icon("../assets/yakuza0.ico");

    println!("cargo:rerun-if-changed=interceptor.asm");
    println!("cargo:rustc-env=CARGO_CFG_TARGET_FEATURE=fxsr,sse,sse2,avx");
    cc::Build::new()
        .file("src/interceptor.asm")
        .compile("interceptor");

    res.compile().unwrap();
}
