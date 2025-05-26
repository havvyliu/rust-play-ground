use rust_decimal::Decimal;

#[test]
pub fn test_decimal() {
    let price = Decimal::from_str_exact("1212.0000").unwrap();
    println!("{:?}", price);

    let f = "1212.7200".parse::<f32>().unwrap();
    println!("{:?}", f);

    let dt: f32;
    dt = 0.1;
    println!("{:?}", dt);
}