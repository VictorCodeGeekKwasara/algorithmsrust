use std::collections::HashMap;
use rand::seq::IteratorRandom;

#[derive(Debug)]
pub struct Graph {

  pub table:HashMap<u64,Vec<u64>>,

}

impl Graph {

  pub fn new()-> Graph{
    Graph { table:HashMap::new() }
  }

  pub  fn add_node(&mut self,nd:u64){

   match self.table.get(&nd) {

    None => {
      self.table.insert(nd,Vec::new());
    },
    _ =>{} 

    }
  }

  pub fn add_edge(&mut self, node_a:u64, node_b:u64){

    self.add_node(node_a);
    self.add_node(node_b);

    let  map =&mut  self.table.entry(node_a).and_modify(|e| e.push(node_b));

  }

  pub fn get_index(&self,node: u64,adj: u64)-> Option<usize> {
      match self.table.get(&node) {
        Some(v)=> {
         if let Some(indx)= v.iter().position(|&x| x == adj){
            return Some(indx)
         }
         return None
        },
        None => None
      }  

  }

  pub fn remove_edge(&mut self, node_a:u64, node_b:u64){
    // (remove all a from b)* and all  b from a

    // all b from a 
    while let Some(v) = self.get_index(node_a,node_b){

      self.table.entry(node_a).and_modify(|e| {e.remove(v);});

    } 
    // all a from b
    while let Some(v) = self.get_index(node_b,node_a){

      self.table.entry(node_b).and_modify(|e|{ e.remove(v);});    
    } 
  }

  pub fn join_vertex (&mut self, node_a:u64, node_b:u64) {

    let v = self.table.get(&node_b).unwrap();
    let v = v.clone() ;

    for val in v {
       self.table.entry(node_a).and_modify(|e|{ e.push(val);});

    }

    self.remove_vertex(node_a, node_b) ;

  }

  pub fn remove_vertex(&mut self,node_a:u64,node_b:u64){

   for val in  self.table.values_mut(){

     *val = val.iter().filter_map(|x|{ 

        if x == &node_b{
          Some(node_a)
        }else{
          Some(*x)
        }

     }).collect()
  }   
  self.table.remove(&node_b);

  }
  pub fn rand_vertex(&self)-> u64 {
    let mut rng = rand::thread_rng();

    // let index =rng.gen_range(1..=self.table.keys().len());
    
    *self.table.keys().choose(&mut rng).unwrap()
  }

  pub fn rand_edge(&self,v:&Vec<u64>)-> u64 {
    let mut rng = rand::thread_rng();
    // let val = rng.gen_range(0..v.len());
    *v.iter().choose(&mut rng).unwrap() 
  }

  pub fn mincut(&mut self) -> u64 {

    let len = self.table.keys().len()-2 ;

     let mut  counter = 0 ;

    while counter < len {
    let node_a = self.rand_vertex();
    let node_adj_vec = self.table.get(&node_a).unwrap() ;
    let node_b = self.rand_edge(node_adj_vec);

    println!("cutting node:{} and node:{}", node_a, node_b);

    self.remove_edge(node_a, node_b);

    self.join_vertex(node_a,node_b);

    counter = counter + 1 ;
    }  

    self.count_edges()
  } 

  pub fn rm_duplicates (&mut self,node:u64){
    self.table.entry(node).and_modify(|e| {e.sort();e.dedup()});
  }

  pub fn count_edges(&self)-> u64 {

    self.table.values().next().unwrap().len() as u64
  }
}

