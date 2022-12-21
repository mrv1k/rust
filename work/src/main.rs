#[allow(unused_variables)]
fn main() {
    // let mut x = vec![1, 2, 3];
    // let mut y = vec![4, 5, 6];
    // let z = x.append(&mut y);
    let mut x = vec![1, 2, 3];
    let mut y = vec![4, 5, 6];
    // let z = x.drain(1..);
    // x.append(y.drain(1..));
    x.extend(y.drain(1..));
    println!("Hello, world!, {:?}", x);

    let mut a = vec![0u8; 1024];
    let mut b = Vec::new();
    b.extend(a.drain(..));

    let mut c = vec![0u8; 1024];
    let mut d = Vec::new();
    c.append(&mut d);

    // let mut v = vec![1, 2, 3];
    // let u: Vec<_> = v.drain(1..).collect();
    // assert_eq!(v, &[1]);
    // assert_eq!(u, &[2, 3]);

    // // A full range clears the vector, like `clear()` does
    // v.drain(..);
    // assert_eq!(v, &[]);

    // let mut vec = vec![1, 2, 3];
    // let mut vec2 = vec![4, 5, 6];
    // vec.append(&mut vec2);
    // assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
    // assert_eq!(vec2, []);
}
