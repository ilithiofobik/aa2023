mod leader;
mod experiments;
mod plot;
mod stats;

fn main() {
    let u = 128;
    let n = 128;
    let l = 7;
    let num_of_exps = 1_000_000;

    let exact_exp = experiments::exact_experiment(n, num_of_exps);
    let bound_exp_2 = experiments::bound_experiment(2, u, num_of_exps);
    let bound_exp_half = experiments::bound_experiment(u/2, u, num_of_exps);
    let bound_exp_u = experiments::bound_experiment(u, u, num_of_exps);

    plot::plot_histogram(&exact_exp, n, None, num_of_exps).unwrap();
    plot::plot_histogram(&bound_exp_2, 2,  Some(u), num_of_exps).unwrap();
    plot::plot_histogram(&bound_exp_half, u/2, Some(u), num_of_exps).unwrap();
    plot::plot_histogram(&bound_exp_u, u, Some(u), num_of_exps).unwrap();

    println!("Expected value for exact experiment: {}", stats::expected_value(&exact_exp, num_of_exps));
    println!("Variation for exact experiment: {}", stats::variation(&exact_exp, num_of_exps));

    let round_exp_2 = experiments::round_experiment(2, l, num_of_exps);
    let round_exp_half = experiments::round_experiment(u/2, l, num_of_exps);
    let round_exp_u = experiments::round_experiment(u, l, num_of_exps);

    println!("Round success for n={}, u={}: {}", 2, u, round_exp_2);
    println!("Round success for n={}, u={}: {}", u/2, u, round_exp_half);
    println!("Round success for n={}, u={}: {}", u, u, round_exp_u);
}
