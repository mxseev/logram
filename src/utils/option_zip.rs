pub fn option_zip<A, B>(a: Option<A>, b: Option<B>) -> Option<(A, B)> {
    a.and_then(|a| b.map(|b| (a, b)))
}

#[cfg(test)]
mod tests {
    use super::option_zip;

    #[test]
    fn main() {
        assert_eq!(option_zip::<(), ()>(None, None), None);
        assert_eq!(option_zip::<u8, u8>(Some(255), None), None);
        assert_eq!(option_zip::<u8, u8>(None, Some(255)), None);
        assert_eq!(
            option_zip::<bool, u8>(Some(true), Some(255)),
            Some((true, 255))
        );
    }
}
