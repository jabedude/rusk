use cc;
use glob;


fn main() {
    let mut compiler = cc::Build::new();

    compiler.include(".");

    let globs = &[
        "tsk/auto/*.c",
        "tsk/base/*.c",
        "tsk/fs/*.c",
        "tsk/hashdb/*.c",
        "tsk/img/*.c",
        "tsk/pool/*.c",
        "tsk/sorter/*.c",
        "tsk/util/*.c",
        "tsk/vs/*.c",
    ];

    for pattern in globs {
        for path in glob::glob(pattern).unwrap() {
            let path = path.unwrap();
            compiler.file(path);
        }
    }

    compiler.compile("libtsk.a");
}
