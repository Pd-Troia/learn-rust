mod pure_split;
fn main() {
    let str = String::from("maria antonieta de lima");
    let char = ' ';
    let vec = pure_split::pure_split(&str,char);
    println!("{:?}",vec);
}
