pub fn raindrops(n: u32) -> String {
    let mut res: String = "".to_owned();
    if n%3 == 0 {
        res = format!("{}{}", res, "Pling");
    }
    if n%5 == 0{
        res = format!("{}{}", res, "Plang");
    }
    if n%7 == 0{
        res = format!("{}{}", res, "Plong");
    }
    if res.len() == 0 {
        res = format!("{}{}", res, n);
    }
    res.to_owned()
}
