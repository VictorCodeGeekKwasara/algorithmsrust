use std::str::FromStr ;
use num_bigint::{BigInt};

fn _karatsuba(a:BigInt , b:BigInt)-> BigInt {

    let mut a_str = format!("{}",a);
    let mut b_str = format!("{}",b);

    let a_len = a_str.len() ;
    let b_len = b_str.len() ;


    if a_len == 1 && b_len ==1 {
        return a*b
    }

    if a_len < b_len {

        let gap = b_len - a_len ;
        // how to to pad x 00 befor a_str
        a_str = "0".repeat(gap) + &a_str ; 
    } 
    if b_len < a_len {

        let gap = a_len - b_len ;
        // how to to pad x 00 befor b_str
        b_str = "0".repeat(gap) + &b_str ; 
    } 

    let len = a_str.len();
    let mut pow:u32 = len as u32 ;
    if len%2!=0 {
        pow = len as u32-1 ;
    }
    let mid = (len as f32/2.0).ceil() as usize ;
    let mid_pow = (len as f32/2.0).floor() as u32 ;

    let a = _to_big_int(&a_str[..mid]);
    let b = _to_big_int(&a_str[mid..]);
    let c = _to_big_int(&b_str[..mid]);
    let d = _to_big_int(&b_str[mid..]);

    let ac = _karatsuba(a.clone(),c.clone());
    let bd = _karatsuba(b.clone(),d.clone());
    let mid_term = _karatsuba(a+b, c+d) - ac.clone() -bd.clone() ; 

    let ten = BigInt::from(10);

    return ten.pow(pow)*ac + ten.pow(mid_pow )* mid_term + bd  


}

pub fn _multiply_string(x_str:&str, y_str:&str) -> BigInt {
    let x=_to_big_int(x_str);
    let y=_to_big_int(y_str);
    return _karatsuba(x,y);

}

pub fn _to_big_int(x:&str)->BigInt{
    BigInt::from_str(x).unwrap() 
}

#[cfg(test)]
mod tests {

    use super::* ;
    #[test]
    fn multiply_works () {

        let num1 = "12123456781234567891234567891234567891234512345678";
        let num2 = "11234567823456789123456123451234562234567890234567890";

        let big_num1  = _to_big_int(num1);
        let big_num2  = _to_big_int(num2);

        assert_eq!(_multiply_string(num1, num2),BigInt::from(big_num1*big_num2))
    }
}