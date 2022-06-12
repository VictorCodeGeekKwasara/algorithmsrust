use std::str::FromStr ;
use num_bigint::{BigInt};

fn karatsuba(a:BigInt , b:BigInt)-> BigInt {

    let mut a_str = format!("{}",a);
    let mut b_str = format!("{}",b);

    let a_len = a_str.len() ;
    let b_len = b_str.len() ;


    if a_len == 1 && b_len ==1 {
        return a*b
    }

    if a_len < b_len {

        let gap = b_len - a_len ;
        // how to to pad x 00 before a_str
        a_str = "0".repeat(gap) + &a_str ; 
    } 
    if b_len < a_len {

        let gap = a_len - b_len ;
        // how to to pad x 00 before b_str
        b_str = "0".repeat(gap) + &b_str ; 
    } 

    let len = a_str.len();
    let mid = len/2 ;

    let a = BigInt::from_str(&a_str[..mid]).unwrap();
    let b = BigInt::from_str(&a_str[mid..]).unwrap();
    let c = BigInt::from_str(&b_str[..mid]).unwrap();
    let d = BigInt::from_str(&b_str[mid..]).unwrap();


    let ac = karatsuba(a.clone(),c.clone());
    let bd = karatsuba(b.clone(),d.clone());
    let mid_term = karatsuba(a+b, c+d) - ac.clone() -bd.clone() ; 

    let ten = BigInt::from(10);

    return ten.pow(len as u32)*ac + ten.pow(mid as u32 )* mid_term + bd  


}

pub fn multiply_string(x_str:&str, y_str:&str) -> BigInt {
    let x=BigInt::from_str(x_str).unwrap();
    let y=BigInt::from_str(y_str).unwrap();
    return karatsuba(x,y);

}

#[cfg(test)]
mod tests {

    use super::* ;
    #[test]
    fn multiply_works () {

        let num1 = "12123456781234567891234567891234567891234512345678";
        let num2 = "11234567823456789123456123451234562";

        assert_eq!(multiply_string(num1, num2),BigInt::from(12*12))
    }
}