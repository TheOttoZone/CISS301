use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    // Struct that pairs a set and a relation for the set
    struct SetRelation<'a>{
        set: HashSet<&'a str>,
        relation: HashSet<(&'a str, &'a str)>
    }

    // All the set + relaton pairs stored in one vector
    let set_relations:Vec<SetRelation> = vec![
        SetRelation{set: HashSet::from(["a", "b", "c", "d", "e", "f"]), relation: HashSet::from([("a", "b"), ("a", "e"), ("b", "c"), ("c", "d"), ("d", "f")])},
        SetRelation{set: HashSet::from(["1", "3", "5", "7", "8", "12", "13"]), relation: HashSet::from([("1", "8"), ("1", "12"), ("3", "8"), ("3", "12"), ("5", "8"), ("5", "12"), ("7", "8"), ("7", "12")])},
        SetRelation{set: HashSet::from(["()","!", "*", "+", "<<","<", "&"]), relation: HashSet::from([("+", "*"),("*", "()"),("*", "!"),("+", "<<"),("<<", "<"),("<<", "&")])},
        SetRelation{set: HashSet::from(["m", "d", "b", "s", "g", "f"]), relation: HashSet::from([("m", "b"), ("m", "s"), ("d", "b"), ("d", "s"), ("g", "m"), ("f", "m")])},
        SetRelation{set: HashSet::from(["a", "b", "c", "d", "f"]), relation: HashSet::from([("a", "a"), ("b", "b"),("c", "c"),("d", "d")])},
        SetRelation{set: HashSet::from(["1", "2", "3", "4"]), relation: HashSet::from([("1", "2"), ("1", "1"), ("2", "2"),("3", "3"),("4", "4")])},
        SetRelation{set: HashSet::from(["()", "&"]), relation: HashSet::from([("()", "()"), ("&", "&")])}
    ];

    // Iterate through the vector to test each set + relation pair
    for (index, i) in set_relations.iter().enumerate() {
        println!("--- Test Case {}: ---", index + 1);
        println!("Set: {:?}", i.set);
        println!("Relation: {:?}", i.relation);
        println!("Reflexive: {}", reflexive(&i.set, &i.relation));
        println!("Symmetric: {}", symmetric(&i.set, &i.relation));
        println!("Transitive: {}", transitive(&i.set, &i.relation));
        println!("Anti-Symmetric: {}", antisymmetric(&i.set, &i.relation));
        println!("Equivalence relation: {}\n", equivalence(&i.set, &i.relation));
    }


    
}

fn reflexive<T: std::fmt::Debug + Eq + Hash + Clone>(set: &HashSet<T>, relation: &HashSet<(T,T)>) -> bool{
    for a in set{
        // if the function doesnt contain a,a its not reflexive
        if !relation.contains(&(a.clone(), a.clone())){
            return false;
        }
    }
    true
}

fn symmetric<T: std::fmt::Debug + Eq + Hash + Clone>(set: &HashSet<T>, relation: &HashSet<(T,T)>) -> bool{
    for a in set{
        for b in set{
            if relation.contains(&(a.clone(), b.clone())){
                // only check for not containing the opposite because only then is it not symmetric.
                // if this was checking for if it did contain it then it would be true on the first iteration and skip subsequent instances where there may not be a matching opposite pair.
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
                    if !relation.contains(&(a.clone(), c.clone())){ // again, only check if the most specific part of the test is not contained to test transitivity
                        return false;
                    }
                }
            }

        }
    }
    true
}

fn antisymmetric<T: std::fmt::Debug + Eq + Hash + Clone>(set: &HashSet<T>, relation: &HashSet<(T,T)>) -> bool{
    for a in set{
        for b in set{
            if relation.contains(&(a.clone(), b.clone())){
                if relation.contains(&(b.clone(), a.clone())) && b != a{ // test for the opposite of symmetric but also require that the values arent equivelant
                    return false;
                }
            }
        }
    }
    true
}

fn equivalence<T: std::fmt::Debug + Eq + Hash + Clone>(set: &HashSet<T>, relation: &HashSet<(T,T)>) -> bool{
    if reflexive(set, relation) && symmetric(set, relation) && transitive(set, relation){ // test for all
        return true;
    }
    false
}