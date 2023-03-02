pub fn expected_value(data: &Vec<(usize, usize)>, num_of_exps: usize) -> f64 {
    let sum = data.iter().map(|(x, y)| (*x) * (*y)).sum::<usize>() as f64;
    sum / (num_of_exps as f64)
}
   
pub fn variation(data: &Vec<(usize, usize)>, num_of_exps: usize) -> f64 {
    let expected = expected_value(data, num_of_exps);
    let expected_square = data.iter().map(|(x, y)| (*x as f64).powi(2) * (*y as f64)).sum::<f64>() / (num_of_exps as f64);
    expected_square - (expected * expected)
}