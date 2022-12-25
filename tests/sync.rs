use sync_mv::digest;

#[test]
fn sync() {
    let summary = digest::get(".").unwrap();
    for (path, size) in summary {
        println!("{}, {}", size, path)
    }
}
