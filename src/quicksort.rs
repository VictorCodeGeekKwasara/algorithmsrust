fn _quicksort(vec:Vec<u64>,low:usize,high:usize)->Vec<u64> {

  if high - low <= 1 {
    return vec
  }

  let vec = vec ;
  let pivot = low ;
  let  (mut vec,mid) = _partition(vec, pivot, low, high);

  if mid <= high {

   vec = _quicksort(vec, mid+1 , high) ;
    
   vec = _quicksort(vec,low,mid);

  }  

  vec
}



fn _partition(vec:Vec<u64>,pivot:usize,low:usize,high:usize)-> (Vec<u64>, usize){

  let mut vec = vec ;

  let mut pivot = pivot ;

  if pivot != low {
    vec.swap(pivot,low);
    pivot = low ;
  }

  let pivot_val = vec[pivot] ; 
  let mut  fin_pos = pivot + 1 ;

  let mut loop_var = fin_pos;

  while loop_var < high {

    if pivot_val >= vec[loop_var]{

      vec.swap(fin_pos,loop_var);

      fin_pos =fin_pos + 1 ;

    }

    loop_var = loop_var + 1 ;
  } 

  let fin_pos = fin_pos -1 ;
  vec.swap(fin_pos,pivot);

  (vec,fin_pos)

}

#[cfg(test)]
mod tests {

  use super::* ;

  #[test]
  fn quicksort_works(){
    let vec = vec![1,2,3,3,2,4,6,6,6,6,6,6,6,6,3,3,3,3,1,1,1,1,4,4,4,7,0,0,0,10,20,200,201,99];
    let len = vec.len() ;
    assert_eq!(_quicksort(vec,0,len),vec![0,0,0,1,1,1,1,1,2,2,3,3,3,3,3,3,4,4,4,4,6,6,6,6,6,6,6,6,7,10,20,99,200,201]);
  }                                  
  #[test]
  fn partition_works() {
    let vec = vec![5,4,2,1,3,6];
    let len = vec.len() ;
                              
    assert_eq!(_partition(vec,4,0,len).0[2],3);
  }

}