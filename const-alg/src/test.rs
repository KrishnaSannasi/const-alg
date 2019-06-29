use crate::{Matrix, One, SquareMatrix, Zero};

#[test]
fn test_mul() {
    let mut m = Matrix([[1, 2], [3, 4]]);

    assert_eq!(m * m, Matrix([[7, 10], [15, 22]]));

    m *= m;

    assert_eq!(m, Matrix([[7, 10], [15, 22]]));

    m += m;

    assert_eq!(m, Matrix([[14, 20], [30, 44]]));

    m += One::one();
    m += Zero::zero();

    assert_eq!(m, Matrix([[15, 20], [30, 45]]));

    m *= SquareMatrix::<_, 2>::one();

    assert_eq!(m, Matrix([[15, 20], [30, 45]]));

    assert_eq!(m * Matrix([[1, 1]]).T(), Matrix([[35, 75]]).T());

    assert_eq!(Matrix([[1, 1]]) * m, Matrix([[45, 65]]));

    assert_eq!(-m, Matrix([[-15, -20], [-30, -45]]));

    m *= Matrix::<_, 2, 2>::zero();

    assert_eq!(m, Zero::zero());
}

#[test]
fn test_iter() {
    fn is_exauasted<I: Iterator>(i: I) {
        assert!({i}.next().is_none());
    }

    let mut m = Matrix([[1, 2, 3], [3, 4, 5], [5, 6, 7]]);

    let mut iter = m.into_rows();

    assert_eq!(iter.nth(0).unwrap().collect::<Vec<_>>(), [1, 2, 3]);

    assert_eq!(iter.nth(1).unwrap().collect::<Vec<_>>(), [5, 6, 7]);
    
    is_exauasted(iter);

    let mut iter = m.into_cols();

    assert_eq!(iter.nth(0).unwrap().collect::<Vec<_>>(), [1, 3, 5]);

    assert_eq!(iter.nth(1).unwrap().collect::<Vec<_>>(), [3, 5, 7]);
    
    is_exauasted(iter);

    let mut iter = m.rows_mut();

    assert_eq!(
        iter.nth(0).unwrap().collect::<Vec<_>>(),
        [&mut 1, &mut 2, &mut 3]
    );

    assert_eq!(
        iter.nth(1).unwrap().collect::<Vec<_>>(),
        [&mut 5, &mut 6, &mut 7]
    );
    
    is_exauasted(iter);

    let mut iter = m.cols_mut();

    assert_eq!(
        iter.nth(0).unwrap().collect::<Vec<_>>(),
        [&mut 1, &mut 3, &mut 5]
    );

    assert_eq!(
        iter.nth(1).unwrap().collect::<Vec<_>>(),
        [&mut 3, &mut 5, &mut 7]
    );
    
    is_exauasted(iter);

    let mut iter = m.rows();

    assert_eq!(iter.nth(0).unwrap().collect::<Vec<_>>(), [&1, &2, &3]);

    assert_eq!(iter.nth(1).unwrap().collect::<Vec<_>>(), [&5, &6, &7]);
    
    is_exauasted(iter);

    let mut iter = m.cols();

    assert_eq!(iter.nth(0).unwrap().collect::<Vec<_>>(), [&1, &3, &5]);

    assert_eq!(iter.nth(1).unwrap().collect::<Vec<_>>(), [&3, &5, &7]);
    
    is_exauasted(iter);

    let mut iter = m.into_rows().rev();

    assert_eq!(iter.nth(0).unwrap().rev().collect::<Vec<_>>(), [7, 6, 5]);

    assert_eq!(iter.nth(1).unwrap().rev().collect::<Vec<_>>(), [3, 2, 1]);
    
    is_exauasted(iter);

    let mut iter = m.into_cols().rev();

    assert_eq!(iter.nth(0).unwrap().rev().collect::<Vec<_>>(), [7, 5, 3]);

    assert_eq!(iter.nth(1).unwrap().rev().collect::<Vec<_>>(), [5, 3, 1]);
    
    is_exauasted(iter);

    let mut iter = m.rows_mut().rev();

    assert_eq!(
        iter.nth(0).unwrap().rev().collect::<Vec<_>>(),
        [&mut 7, &mut 6, &mut 5]
    );

    assert_eq!(
        iter.nth(1).unwrap().rev().collect::<Vec<_>>(),
        [&mut 3, &mut 2, &mut 1]
    );
    
    is_exauasted(iter);

    let mut iter = m.cols_mut().rev();

    assert_eq!(
        iter.nth(0).unwrap().rev().collect::<Vec<_>>(),
        [&mut 7, &mut 5, &mut 3]
    );

    assert_eq!(
        iter.nth(1).unwrap().rev().collect::<Vec<_>>(),
        [&mut 5, &mut 3, &mut 1]
    );
    
    is_exauasted(iter);

    let mut iter = m.rows().rev();

    assert_eq!(iter.nth(0).unwrap().rev().collect::<Vec<_>>(), [&7, &6, &5]);

    assert_eq!(iter.nth(1).unwrap().rev().collect::<Vec<_>>(), [&3, &2, &1]);
    
    is_exauasted(iter);

    let mut iter = m.cols().rev();

    assert_eq!(iter.nth(0).unwrap().rev().collect::<Vec<_>>(), [&7, &5, &3]);

    assert_eq!(iter.nth(1).unwrap().rev().collect::<Vec<_>>(), [&5, &3, &1]);
    
    is_exauasted(iter);
}

#[test]
fn test_get_all_mut() {
    let mut m = Matrix([[0, 1, 2, 3], [4, 5, 6, 7]]);

    let all = m.get_all_mut([(0, 0), (0, 2), (2, 0), (0, 0)]);

    assert_eq!(all, [Some(&mut 0), Some(&mut 2), None, None]);
}
