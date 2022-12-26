use sync_mv::digest;

#[test]
fn sync() {
    let src_summary = digest::get("tests/test_data/src").unwrap();
    for (path, metadata) in &src_summary {
        println!("{}, {:?}", path, metadata);
    }

    let dst_summary = digest::get("tests/test_data/dst").unwrap();
    for (path, metadata) in &dst_summary {
        println!("{}, {:?}", path, metadata);
    }

    digest::diff(&src_summary, &dst_summary);
}
