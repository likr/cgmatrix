fn main() {
    let camera = cgmatrix::viewing_matrix([0.0, 0.0, 200.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]);
    let theta = 0.5;
    let mv_matrix = cgmatrix::matmul(cgmatrix::rotate_x(theta), camera);
    println!("{:?}", mv_matrix);
}
