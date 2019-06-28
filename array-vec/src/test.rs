use super::{ArrayVec, IntoIter};

#[test]
fn iter() {
    let a = ArrayVec::<_, {5}>::from([10, 20, 30, 40, 50]);

    let mut iter = a.clone().into_iter();

    assert_eq!(iter.next(), Some(10));
    
    assert_eq!(iter.nth(0), Some(20));
    assert_eq!(iter.nth(2), Some(50));

    let mut iter = a.into_iter();

    assert_eq!(iter.next_back(), Some(50));
    
    assert_eq!(iter.nth_back(0), Some(40));
    assert_eq!(iter.nth_back(2), Some(10));
}
