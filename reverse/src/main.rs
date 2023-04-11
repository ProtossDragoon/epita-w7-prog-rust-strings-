fn main()
{
    let s = reverse("Hello, world!");
    dbg!(s);
}

fn reverse(s: &str) -> String
{
    return s.chars().rev().collect();
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_reverse()
    {
        assert_eq!(reverse(""), "");
        assert_eq!(reverse("a"), "a");
        assert_eq!(reverse(" "), " ");
        assert_eq!(reverse("가"), "가");
        assert_eq!(reverse("ab"), "ba");
        assert_eq!(reverse("abcd"), "dcba");
        assert_eq!(reverse("abcd "), " dcba");
        assert_eq!(reverse("가bc d"), "d cb가");
    }
}
