use nalgebra::matrix;
use nalgebra::SMatrix;
use nalgebra::RowSVector;

type Matrix6x6 = SMatrix<f64, 6, 6>;
type RowSVector6 = RowSVector<f64, 6>;
type RowSVector4 = RowSVector<f64, 4>;

pub fn ex12() {
    println!("Exercise 12:");

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
            let mut counter = 0;

            while ((vector * matrix) - vector).norm() > 0.0 {
                vector *= matrix;
                counter += 1;
            }

            println!("{}: alpha = {}, steps = {}", name, alpha, counter);
            println!("Vector:");
            for num in vector.iter() {
                print!("{:.3} ", num);
            }    
            println!("\n");        
        }
    }
}

pub fn ex13() {
    println!("Exercise 13:");

    let pg = 
        matrix![
            0.0, 0.3, 0.1, 0.6;
            0.1, 0.1, 0.7, 0.1;
            0.1, 0.7, 0.1, 0.1;
            0.9, 0.1, 0.0, 0.0;
        ];

    println!("Task a:");
    let mut vector = RowSVector4::from_element(1.0 / 4.0);
    while ((vector * pg) - vector).norm() > 2.0f64.powi(-40) {
        vector *= pg;
    }
    println!("Vector: ");
    for num in vector.iter() {
        print!("{:.3} ", num);
    }
    println!();

    println!("Task b:");
    let vector = RowSVector4::from([1.0, 0.0, 0.0, 0.0]);
    let pg32 = pg.pow(32);
    let pbbs = vector * pg32;
    println!("Pbb of being in state 3 after 32 steps: {}", pbbs[2]);

    println!("Task c:");
    let vector = RowSVector4::from_element(0.25);
    let pg128 = pg.pow(128);
    let pbbs = vector * pg128;
    println!("Pbb of being in state 3 after 32 steps: {}", pbbs[2]);

    println!("Task d:");
    let epsilons = [0.1, 0.01, 0.001];
    for epsilon in epsilons {
        let mut t = 0;
        let mut vector_t = RowSVector4::from([1.0, 0.0, 0.0, 0.0]);
        let mut pg_t = pg;

        let mut maximum = 
            (0..4)
            .map(|s| pg_t[(0, s)] - vector_t[s])
            .fold(0.0f64, |acc, x| acc.max(x));

        while maximum > epsilon {
            t += 1;
            vector_t *= pg;
            pg_t *= pg;
            maximum = 
                (0..4)
                .map(|s| pg_t[(0, s)] - vector_t[s])
                .fold(0.0f64, |acc, x| acc.max(x));
        }

        println!("Epsilon = {}, t = {}", epsilon, t);
    }
}