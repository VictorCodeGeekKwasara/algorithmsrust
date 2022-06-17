
fn _count_inversions(vec: Vec<u64>)->(Vec<u64>,u64){

   let len = vec.len() ;
   let mut inversions = 0 ;
   if len ==1 {
    return (vec,0)
   }
   let mid = len/2 ;
   let left_half:Vec<u64> = vec[..mid].into(); 
   let right_half:Vec<u64> = vec[mid..].into();

   let left_sorted = _count_inversions(left_half);
   inversions = inversions + left_sorted.1 ;
   let right_sorted = _count_inversions(right_half);
   inversions = inversions + right_sorted.1 ;
   let merged = _merge(left_sorted.0, right_sorted.0);
   inversions = inversions + merged.1 ;

   return (merged.0,inversions)
}

fn _merge(vec:Vec<u64>, vec2:Vec<u64>)-> (Vec<u64>,u64){

    let mut inversions = 0 ;
    let mut return_vec:Vec<u64> = Vec::new() ;
    let vec_len = vec.len();
    let vec2_len =vec2.len();
    let tot_len = vec_len + vec2_len ;

    let mut vec_pointer = 0 ;
    let mut vec2_pointer = 0 ;
 
    for _ in 0..tot_len {

        if vec_pointer >= vec_len || vec2_pointer >= vec2_len {
            break
        } 

        let val1 = vec[vec_pointer] ;
        let val2 = vec2[vec2_pointer] ;

        if val1 < val2 {
            return_vec.push(val1);
            vec_pointer = vec_pointer + 1 ;
        }else{
            return_vec.push(val2);
            let gap = (vec_len - vec_pointer ) as u64;
            inversions = inversions + gap  ;
            vec2_pointer = vec2_pointer + 1 ;
            
        }  
    }

    if vec_pointer < vec_len {

        for val in vec_pointer..vec_len {
           return_vec.push(vec[val]); 
        }
    }
    if vec2_pointer < vec2_len {

        for val in vec2_pointer..vec2_len {
           return_vec.push(vec2[val]); 
        }
    }

    (return_vec,inversions)

}


#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn merge_works() {
    let vec = vec![1,2,3,6,7,9] ;
    let sort_vec = vec![1,2,6];
    assert_eq!(_merge(vec, sort_vec).0,vec![1,1,2,2,3,6,6,7,9]);
  }

  #[test]
  fn count_inversions_works() {
     let vec = vec![6,5,4,3,2,1];
     assert_eq!(_count_inversions(vec),(vec![1,2,3,4,5,6],15));
  }
}