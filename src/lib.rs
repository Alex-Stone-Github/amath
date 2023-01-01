mod gsalg;
mod salg;
pub use gsalg::Matrix;
pub use salg::{Vec2, Vec3};

/*
 * Just some notes on the organization of this project. salg stands
 * for sized linear algebra, it's basically just a linear algebra
 * library where all of the types have a predefined number of elments.
 * For gsalg, the name stands for generic sized, this part exposes
 * a generic matrix struct that can be used to hold generic data with
 * an unknown number of elements at compile time.
 */


/* tests  -------------- */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
    #[test]
    fn binary_ops() {
        let a = Matrix::new(3, 1, vec![1.0, 2.0, 3.0]);
        let b = Matrix::new(3, 1, vec![2.0, 3.0, 4.0]);
        let c = Matrix::new(3, 1, vec![3.0, 5.0, 7.0]);
        let product = Matrix::add(&a, &b).expect("This should work");
        println!("{:?}", product);
        assert_eq!(product, c);
    }
    #[test]
    fn matmul_other() {
        let identity = Matrix::new(3, 3, vec![
                                   1.0, 0.0, 0.0,
                                   0.0, 2.0, 0.0,
                                   0.0, 0.0, 1.0]);
        let x = Matrix::new(3, 1, vec![ 1.0, 2.0, 3.0]);
        let y = Matrix::new(3, 1, vec![ 1.0, 4.0, 3.0]);
        assert_eq!(Matrix::matmul(&identity, &x).expect("This should work"), y);
    }
    #[test]
    fn matmul_identity() {
        let identity = Matrix::new(3, 3, vec![
                                   1.0, 0.0, 0.0,
                                   0.0, 1.0, 0.0,
                                   0.0, 0.0, 1.0]);
        let x = Matrix::new(3, 1, vec![ 1.0, 2.0, 3.0]);
        assert_eq!(Matrix::matmul(&identity, &x).expect("This should work"), x);
    }
}



