pub fn gcd( a : i32,  b : i32) -> i32{
    let (mut left, mut right) = (a,b);
    if b > a {
        left = b;
        right = a;
    }
    let mut tmp;
    while right != 0 {
        tmp = right;
        right = left % right;
        left = tmp;
    }
    left
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_gcd(){
        assert_eq!(gcd(5,3), 1);
        assert_eq!(gcd(15,9), 3);
        assert_eq!(gcd(9103452,8785490), 2);
    }
}