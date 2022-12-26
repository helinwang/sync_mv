use sync_mv::digest;

#[test]
fn sync() {
    let src = digest::get("tests/test_data/src").unwrap();
    for (path, metadata) in &src.files {
        println!("{}, {:?}", path, metadata);
    }

    let dst = digest::get("tests/test_data/dst").unwrap();
    for (path, metadata) in &dst.files {
        println!("{}, {:?}", path, metadata);
    }

    println!("{}", digest::diff(&src, &dst));
}
