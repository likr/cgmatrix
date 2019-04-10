# cgmatrix

Simple matrix library for computer graphics in Rust


## About

cgmatrix is a matrix library intended to use with OpenGL and WebGL.
Matrices are represented as float arrays that can be directly passed to OpenGL and WebGL.

## Usage

Add cgmatrix in your Cartgo.toml file.

```toml
[dependencies]
cgmatrix = "0.1"
```

And, in your rust code:

```rust
fn main() {
    let camera = cgmatrix::viewing_matrix([0.0, 0.0, 200.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]);
    let theta = 0.5;
    let mv_matrix = cgmatrix::matmul(cgmatrix::rotate_x(theta), camera);
    println!("{:?}", mv_matrix);
}
```

## API

Following methods are supported in cgmatrix:

### Matrix creation

* zeros()
* identity()
* rotate_x(theta)
* rotate_y(theta)
* translate(x, y, z)
* scale(sx, sy, sz)
* viewing_matrix(eye, up, target)
* orthogonal_matrix(left, right, top, bottom, near, far)
* perspective_matrix(fov, aspect, near, far)

### Matrix multiplication

* matmul

### Vector operation

* cross(v1, v2)
* normalize(v)

## License

MIT
