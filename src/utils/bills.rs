/// 生成订单号
pub fn no() -> String {
    let dt = super::dt::now();
    let mut s = String::from("NU");
    s.push_str(&dt.format("%Y%m%d%H%M%S").to_string());
    s.push_str(&super::random::rand_str(4));
    s
}
