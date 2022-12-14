mod dt_test_utils;
use dt_test_utils::{dtsh, DtPipedResult};

fn assert_two(result: DtPipedResult) {
    assert_eq!("2\n", result.stdout);

    let stderr_lines = result.stderr.split('\n').collect::<Vec<_>>();
    assert!(stderr_lines[0].starts_with("dtsh"));
    assert_eq!("RIP: End of input", stderr_lines[1]);
}

#[test]
pub fn one_plus_one_is_two() {
    let res = dtsh("1 1 + pl");

    assert_two(res);
}

#[test]
pub fn one_plus_one_is_still_two() {
    let res = dtsh("1 1 [ + ] do pl");

    assert_two(res);
}

#[test]
pub fn one_plus_one_is_definitely_two() {
    let res = dtsh("1 [ 1 + ] do pl");

    assert_two(res);
}

#[test]
pub fn one_plus_one_is_positively_two() {
    let res = dtsh("[ 1 ] 2 times + pl");

    assert_two(res);
}

#[test]
pub fn one_plus_one_is_never_not_two() {
    let res = dtsh("[ 1 ] [ 1 ] [ + ] [ concat ] 2 times do pl");

    assert_two(res);
}
