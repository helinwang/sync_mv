use sync_mv::digest;

#[test]
fn sync() {
    let summary = digest::get("tests/test_data/src").unwrap();
    for (path, metadata) in summary {
        println!("{}, {:?}", path, metadata);
    }

    let summary = digest::get("tests/test_data/dst").unwrap();
    for (path, metadata) in summary {
        println!("{}, {:?}", path, metadata);
    }
}
