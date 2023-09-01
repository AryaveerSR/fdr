/// Levenstein Distance Algorithm to find how 'similar' two strings are,
/// used to rank results.
///
/// Note: automatically lowercases the arguments.
pub fn levenstein_distance(a: &str, b: &str) -> u8 {
    if a.is_empty() {
        return b.len() as u8;
    }

    if b.is_empty() {
        return a.len() as u8;
    }

    // Check if the first characters of both string slices are the same, (accounting for different capitalisation).
    // `.expect()` is not nessessary on `.nth()` as we already checked for if the strings are empty
    if a.chars().nth(0).unwrap().to_ascii_lowercase()
        == b.chars().nth(0).unwrap().to_ascii_lowercase()
    {
        return levenstein_distance(&a[1..], &b[1..]);
    }

    // Compute these three values (witchcraft)..
    let a_tailed = levenstein_distance(&a[1..], b);
    let b_tailed = levenstein_distance(a, &b[1..]);
    let a_b_tailed = levenstein_distance(&a[1..], &b[1..]);

    // ..and return the lowest of them + 1.
    a_tailed.min(b_tailed).min(a_b_tailed) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let a = "startling";
        let b = "start";
        assert_eq!(levenstein_distance(a, b), 4);
    }

    #[test]
    fn test_capitalization() {
        let a = "StArTlinG";
        let b = "stARt";
        assert_eq!(levenstein_distance(a, b), 4);
    }

    #[test]
    fn test_equal() {
        let a = "start";
        let b = "START";
        assert_eq!(levenstein_distance(a, b), 0);
    }
}
