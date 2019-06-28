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

    m *= Matrix::<_, 2, 2>::zero();

    assert_eq!(m, Zero::zero());
}

#[test]
fn test_get_all_mut() {
    let mut m = Matrix([[0, 1, 2, 3], [4, 5, 6, 7]]);

    let all = m.get_all_mut([(0, 0), (0, 2), (2, 0), (0, 0)]);

    assert_eq!(all, [Some(&mut 0), Some(&mut 2), None, None]);
}
