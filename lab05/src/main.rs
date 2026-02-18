use std::collections::HashSet;
use std::hash::Hash;

fn main() {
	let c:HashSet<char> = HashSet::from(['a','b','c','d','e','f']);
	//let c_rel: HashSet<(char, char)> = HashSet::from([ ('a','b'), ('a','e'),('b','c'),('c','d'),('d','f')]);
	let c_rel: HashSet<(char, char)> = HashSet::from([ ('a','a'), ('b','b'),('c','c'),('d','d'),('e','e'),('f','f')]);
	//let c_rel: HashSet<(char, char)> = HashSet::from([ ('a','b'), ('b','a'),('a','a'), ('b','b'),('c','c'),('d','d'),('e','e'),('f','f')]);
	//let c_rel: HashSet<(char, char)> = HashSet::from([ ('a','b'), ('b','c'), ('a','c')]);
	let _n:HashSet<i32> = HashSet::from([1, 3, 5, 7, 8, 12, 13]);
	let _o:HashSet<&str> = HashSet::from(["()","!", "*", "+", "<<","<", "&"]);
	let _c2:HashSet<char> = HashSet::from(['m','d','b','s','g', 'f']);

    println!("Reflexive: {}", reflexive(&c, &c_rel));
    println!("Symmetric: {}", symmetric(&c, &c_rel));
    println!("Transitive: {}", transitive(&c, &c_rel));
    
}

fn reflexive<T: std::fmt::Debug + Eq + Hash + Clone>(set: &HashSet<T>, relation: &HashSet<(T,T)>) -> bool{
    let mut count = 0;
    for a in set{
        if relation.contains(&(a.clone(), a.clone())){
            count = count +1;
        }
    }
    if count == set.len(){
        return true;
    }else {
        false
    }
}

fn symmetric<T: std::fmt::Debug + Eq + Hash + Clone>(set: &HashSet<T>, relation: &HashSet<(T,T)>) -> bool{
    for a in set{
        for b in set{
            if relation.contains(&(a.clone(), b.clone())){
                if !relation.contains(&(b.clone(), a.clone())){
                    return false;
                }
            }
        }
    }
    true
}

fn transitive<T: std::fmt::Debug + Eq + Hash + Clone>(set: &HashSet<T>, relation: &HashSet<(T,T)>) -> bool{
    for a in set{
        for b in set{
            for c in set{
                if relation.contains(&(a.clone(), b.clone())) && relation.contains(&(b.clone(), c.clone())){
                    if !relation.contains(&(a.clone(), c.clone())){
                        return false;
                    }
                }
            }

        }
    }
    true
}

fn antisymmetric<T: std::fmt::Debug + Eq + Hash + Clone>(set: &HashSet<T>, relation: &HashSet<(T,T)>) -> bool{
    false
}

fn equivelance<T: std::fmt::Debug + Eq + Hash + Clone>(set: &HashSet<T>, relation: &HashSet<(T,T)>) -> bool{
    false
}