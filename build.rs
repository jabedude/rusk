use cc;
use glob;
use pkg_config;


fn main() {
	pkg_config::probe_library("libvmdk").unwrap();
	pkg_config::probe_library("libbfio").unwrap();
	println!("cargo:rerun-if-changed=tsk/");
	println!("cargo:rustc-link-lib=static=tsk");
	println!("cargo:rustc-link-lib=static=bfio");
	println!("cargo:rustc-link-lib=static=vmdk");
	println!("cargo:rustc-link-lib=static=z");
    let mut c_builder = cc::Build::new();

    c_builder.include(".");
	c_builder.define("HAVE_LIBVMDK", None);

    let globs = &[
        "tsk/auto/*.c",
        //"tsk/auto/*.cpp",
        "tsk/base/*.c",
        "tsk/fs/*.c",
        //"tsk/fs/*.cpp",
        "tsk/hashdb/*.c",
        //"tsk/hashdb/*.cpp",
        "tsk/img/*.c",
        "tsk/img/*.cpp",
        "tsk/pool/*.c",
        //"tsk/pool/*.cpp",
        "tsk/sorter/*.c",
        "tsk/util/*.c",
        //"tsk/util/*.cpp",
        "tsk/vs/*.c",
    ];

    for pattern in globs {
        for path in glob::glob(pattern).unwrap() {
            let path = path.unwrap();
            c_builder.file(path);
        }
    }

	// This is what we're going to need to do to get the cpp to compile:
	// https://github.com/alexcrichton/cc-rs/issues/156
	// https://users.rust-lang.org/t/mixed-c-and-c-build-with-cc-rs/26971
	//
	//c_builder.cpp(true).cpp_link_stdlib("stdc++");
    c_builder.compile("libtsk.a");
}
