fn main() {
    let lgArr: Vec<u32> = (0..1_000_000_000).collect();
    let start = std::time::Instant::now();
    let chunked_arr = lgArr.chunks(100).collect::<Vec<&[u32]>>();
    let end = std::time::Instant::now();
    let time = end.duration_since(start).as_micros();

    println!("{:?}", chunked_arr);
    println!("{}", time);
}