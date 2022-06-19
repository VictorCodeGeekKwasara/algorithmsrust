fn _quicksort(vec:Vec<u64>,low:usize,high:usize)->(Vec<u64>,usize ){

  if high - low <= 1 {
    return (vec,0)
  }

  let vec = vec ;
  let (vec,pivot) = median_piv(vec, low, high) ;
  let  (mut vec,mid,mut comparisons) = _partition(vec, pivot, low, high);

  if mid <= high {

   let output = _quicksort(vec, mid+1 , high) ;
   vec = output.0;
   comparisons = comparisons + output.1 ; 
   let output= _quicksort(vec,low,mid);
   vec = output.0;
   comparisons = comparisons + output.1 ;

  }  

  (vec,comparisons)
}



fn _partition(vec:Vec<u64>,pivot:usize,low:usize,high:usize)-> (Vec<u64>, usize,usize){

  let len = high - low  ;
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

  (vec,fin_pos,len-1)

}

fn _low_piv (vec:Vec<u64>,low:usize,_high:usize)-> (Vec<u64>,usize) {
 (vec,low)
} 
fn _high_piv (vec:Vec<u64>,_low:usize,high:usize)-> (Vec<u64>,usize) {

 (vec,high-1)
} 
 fn median_piv (vec:Vec<u64>,low:usize,high:usize)-> (Vec<u64>,usize) {

  let  low_val = vec[low]; //4
  let  high_val = vec[high-1]; 
  let len = high -low ; 
  let mid;
  let mut  median = low ;

  if len%2==0{
     mid = (len/2) -1 + low ; 
  }else{
     mid = (len)/2 + low ;
  }  

  let mid_val = vec[mid];

  if low_val >= high_val && high_val >= mid_val || mid_val >=high_val && high_val >= low_val{    
     median = high-1;
  }

  if high_val >= low_val && low_val >= mid_val || mid_val >=low_val && low_val >= high_val{   
      median = low;       
  }

  if high_val >= mid_val && mid_val >= low_val || low_val >= mid_val && mid_val >= high_val{
    median = mid ;
  }

  (vec,median)
} 