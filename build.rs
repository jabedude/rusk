use cc;
use glob;


fn main() {
	println!("cargo:rerun-if-changed=tsk/");
	println!("cargo:rustc-link-lib=static=tsk");
    let mut compiler = cc::Build::new();

    compiler.include(".");

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
            compiler.file(path);
        }
    }

	// This is what we're going to need to do to get the cpp to compile:
	// https://github.com/alexcrichton/cc-rs/issues/156
	// https://users.rust-lang.org/t/mixed-c-and-c-build-with-cc-rs/26971
	//
	//compiler.cpp(true).cpp_link_stdlib("stdc++");
    compiler.compile("libtsk.a");
}
