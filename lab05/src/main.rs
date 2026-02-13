use std::collections::HashSet;
use std::hash::Hash;

fn main() {
	let c:HashSet<char> = HashSet::from(['a','b','c','d','e','f']);
	//let c_rel: HashSet<(char, char)> = HashSet::from([ ('a','b'), ('a','e'),('b','c'),('c','d'),('d','f')]);
	let c_rel: HashSet<(char, char)> = HashSet::from([ ('a','a'), ('b','b'),('c','c'),('d','d'),('e','e'),('f','f')]);
	let n:HashSet<i32> = HashSet::from([1, 3, 5, 7, 8, 12, 13]);
	let o:HashSet<&str> = HashSet::from(["()","!", "*", "+", "<<","<", "&"]);
	let c2:HashSet<char> = HashSet::from(['m','d','b','s','g', 'f']);

    reflexive(c, c_rel);
    
}

fn reflexive<T: std::fmt::Debug + Eq + Hash + Clone>(set: HashSet<T>, relation: HashSet<(T,T)>){
    let mut count = 0;
    for a in set{
        if relation.contains(&(a.clone(), a.clone())){
            count = count +1;
        }
    }
    if count == relation.len(){
        println!("Reflexive!");
    }
}