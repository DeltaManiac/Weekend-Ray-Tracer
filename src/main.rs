fn main() {
    let x = 200;
    let y = 100;
    println!("P3\n{} {}\n255", x, y);
    for j in (0..=y).rev() {
        for i in 0..x {
            let r = i as f64 / x as f64;
            let g = j as f64 / y as f64;
            let b = 0.2;
            let ir = (255.99 * r) as u16;
            let ig = (255.99 * g) as u16;
            let ib = (255.99 * b) as u16;
            println!("{} {} {}", ir, ig, ib)
        }
    }
}
