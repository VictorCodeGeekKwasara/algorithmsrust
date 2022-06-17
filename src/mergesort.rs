use std::f32::INFINITY;

fn _merge_sort(vec: Vec<u64>)->Vec<u64>{

   let len = vec.len() ;

   if len ==1 {
    return vec
   }
   let mid = len/2 ;
   let left_half:Vec<u64> = vec[..mid].into(); 
   let right_half:Vec<u64> = vec[mid..].into();

   let left_sorted = _merge_sort(left_half);
   let right_sorted = _merge_sort(right_half);

   return _merge(left_sorted, right_sorted)

}

fn _merge(vec:Vec<u64>, vec2:Vec<u64>)-> Vec<u64>{

    let mut return_vec = Vec::new() ;
    let vec_len = vec.len();
    let vec2_len =vec2.len();
    let tot_len = vec_len + vec2_len ;

    let mut vec_pointer = 0 ;
    let mut vec2_pointer = 0 ;
    let mut loop_var = 0 ;

    while loop_var<tot_len {  
      
      let mut a = INFINITY as u64 ;
      let mut b = INFINITY as u64 ;

      if vec_pointer < vec_len {
        a = vec[vec_pointer];        
      }              
      if vec2_pointer < vec2_len {
        b = vec2[vec2_pointer];        
      } 
      
      if a < b {
        return_vec.push(a);
        vec_pointer =vec_pointer + 1 ;
      }else{
        return_vec.push(b);
        vec2_pointer =vec2_pointer + 1;
      }
      loop_var = loop_var + 1 ;
    }

    return_vec

}


#[cfg(test)]
mod tests {

    use super::* ;
    #[test]
    fn merge_works () {

        let vec = vec![1,2,3,6,7,9] ;
        let sort_vec = vec![1,2,6];

        assert_eq!(_merge(vec, sort_vec),vec![1,1,2,2,3,6,6,7,9]);
    }
    #[test]
    fn merge_sort_works(){

        let vec = vec![6,5,4,3,2,1];

        assert_eq!(_merge_sort(vec),vec![1,2,3,4,5,6]);
    }
}