pub fn last4n(s: impl ToString) -> String {
    let s = s.to_string();
    match s.len() <= 4 {
        true => format!("xxx{}", s),
        _ => format!("xxx{}", &s[s.len() - 4..]),
    }
}

#[cfg(test)]
#[test]
fn it_works() {
    assert_eq!(last4n("123456789"), "xxx6789");
    assert_eq!(last4n(""), "xxx");
}
