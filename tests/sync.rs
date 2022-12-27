use assert_cmd::Command;
use temp_file;

#[test]
fn sync() {
    let src_json = r#"{
  "base_dir": "tests/test_data/src/",
  "min_file_bytes": 0,
  "files": {
    "1/1_.txt": {
      "size": 2,
      "modified": 1671944003613389535
    },
    "2_.txt": {
      "size": 2,
      "modified": 1671944011717239623
    },
    "3.txt": {
      "size": 2,
      "modified": 1672027351079920913
    },
    "3_.txt": {
      "size": 2,
      "modified": 1672027351079920913
    },
    "4.txt": {
      "size": 2,
      "modified": 1672027285744982980
    }
  }
}
"#;

    let dst_json = r#"{
  "base_dir": "tests/test_data/dst/",
  "min_file_bytes": 0,
  "files": {
    "1.txt": {
      "size": 2,
      "modified": 1671944003613389535
    },
    "2.txt": {
      "size": 2,
      "modified": 1671944011717239623
    },
    "3.txt": {
      "size": 2,
      "modified": 1672027351079920913
    }
  }
}
"#;
    let assert = Command::cargo_bin("sync_mv")
        .unwrap()
        .args(&[
            "--action",
            "digest",
            "--folder",
            "tests/test_data/src",
            "--min-file-size",
            "0",
        ])
        .assert();
    assert.success().stdout(src_json);

    let assert = Command::cargo_bin("sync_mv")
        .unwrap()
        .args(&[
            "--action",
            "digest",
            "--folder",
            "tests/test_data/dst",
            "--min-file-size",
            "0",
        ])
        .assert();
    assert.success().stdout(dst_json);

    let src = temp_file::with_contents(src_json.as_bytes());
    let dst = temp_file::with_contents(dst_json.as_bytes());

    let assert = Command::cargo_bin("sync_mv")
        .unwrap()
        .args(&[
            "--action",
            "diff",
            "--src",
            src.path().to_str().unwrap(),
            "--dst",
            dst.path().to_str().unwrap(),
        ])
        .assert();

    assert.success().stdout(
        r#"set -x
set -e
mkdir -p 'tests/test_data/dst/1'
mv 'tests/test_data/dst/1.txt' 'tests/test_data/dst/1/1_.txt'
mkdir -p 'tests/test_data/dst'
mv 'tests/test_data/dst/2.txt' 'tests/test_data/dst/2_.txt'
"#,
    );
}
