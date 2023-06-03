use nalgebra::matrix;
use nalgebra::SMatrix;
use nalgebra::RowSVector;

type Matrix6x6 = SMatrix<f64, 6, 6>;
type RowSVector6 = RowSVector<f64, 6>;

pub fn ex12() {
    let pg = 
        matrix![
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0;
            0.0, 0.0, 0.5, 0.0, 0.5, 0.0;
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0;
            0.0, 0.5, 0.0, 0.0, 0.5, 0.0;
            0.0, 0.0, 0.0, 1.0, 0.0, 0.0;
            0.0, 0.0, 1.0, 0.0, 0.0, 0.0;
        ];

    let pg_ne = 
        matrix![
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0;
            0.0, 0.0, 0.0, 0.0, 1.0, 0.0;
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0;
            0.0, 0.5, 0.0, 0.0, 0.5, 0.0;
            0.0, 0.0, 0.0, 1.0, 0.0, 0.0;
            0.0, 0.0, 1.0, 0.0, 0.0, 0.0;
        ];

    let one = Matrix6x6::from_element(1.0 / 6.0);

    let alphas = [0.0, 0.15, 0.5, 1.0];
    let name_pgs = [("with 2-3 edge", pg), ("without 2-3 edge", pg_ne)];

    for (name, pg) in name_pgs {
        for alpha in alphas {
            let mut vector = RowSVector6::from_element(1.0 / 6.0);
            let matrix = (1.0 - alpha) * pg + alpha * one;
            let mut new_vector = vector * matrix;
            let mut counter = 0;

            while (new_vector - vector).norm() > 0.0 {
                vector = new_vector;
                new_vector = vector * matrix;
                counter += 1;
            }

            println!("{}: alpha = {}, steps = {}", name, alpha, counter);
        }
    }
}