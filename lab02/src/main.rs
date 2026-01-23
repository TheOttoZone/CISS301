use std::fmt;
use std::io::{self, BufRead};
use std::collections::HashSet;

// ============================================================================
// DATA STRUCTURES
// ============================================================================

/// Represents a logical expression in propositional logic
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Expr {
    Var(String),                          // Variable (e.g., "p", "q")
    Not(Box<Expr>),                       // Negation (~A)
    And(Box<Expr>, Box<Expr>),            // Conjunction (A * B)
    Or(Box<Expr>, Box<Expr>),             // Disjunction (A + B)
    Implies(Box<Expr>, Box<Expr>),        // Implication (A > B)
}

/// Display implementation for pretty-printing expressions
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Var(s) => write!(f, "{}", s),
            Expr::Not(e) => write!(f, "~{}", e),
            Expr::And(l, r) => write!(f, "({} * {})", l, r),
            Expr::Or(l, r) => write!(f, "({} + {})", l, r),
            Expr::Implies(l, r) => write!(f, "({} > {})", l, r),
        }
    }
}

/// Represents a fact in our knowledge base with its derivation history
#[derive(Clone)]
struct Fact {
    expr: Expr,      // The logical expression
    source: Source,  // How this fact was derived
    id: usize,       // Unique identifier for dependency tracking
}

/// Tracks how a fact was derived (either a premise or derived from other facts)
#[derive(Clone, Debug)]
enum Source {
    Premise,
    Derived { 
        rule: String,         // Name of the inference rule used
        parents: Vec<usize>   // IDs of facts used to derive this one
    },
}

// ============================================================================
// PARSER
// ============================================================================

/// Recursive descent parser for propositional logic expressions
/// Grammar:
///   expr    -> implies
///   implies -> or ('>' implies)?
///   or      -> and ('+' and)*
///   and     -> not ('*' not)*
///   not     -> '~' not | primary
///   primary -> '(' expr ')' | variable
struct Parser {
    tokens: Vec<char>,  // Input tokenized as characters
    pos: usize,         // Current position in token stream
}

impl Parser {
    /// Create a new parser from input string (whitespace is filtered out)
    fn new(input: &str) -> Self {
        Parser { 
            tokens: input.chars().filter(|c| !c.is_whitespace()).collect(), 
            pos: 0 
        }
    }

    /// Look at the current token without consuming it
    fn peek(&self) -> Option<char> {
        self.tokens.get(self.pos).copied()
    }

    /// Consume and return the current token
    fn consume(&mut self) -> Option<char> {
        let c = self.peek();
        if c.is_some() { self.pos += 1; }
        c
    }

    /// Entry point: parse a complete expression
    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_implies()
    }

    /// Parse implication (lowest precedence, right-associative)
    fn parse_implies(&mut self) -> Result<Expr, String> {
        let lhs = self.parse_or()?;
        if self.peek() == Some('>') {
            self.consume();
            Ok(Expr::Implies(Box::new(lhs), Box::new(self.parse_implies()?)))
        } else {
            Ok(lhs)
        }
    }

    /// Parse disjunction (OR, left-associative)
    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_and()?;
        while self.peek() == Some('+') {
            self.consume();
            lhs = Expr::Or(Box::new(lhs), Box::new(self.parse_and()?));
        }
        Ok(lhs)
    }

    /// Parse conjunction (AND, left-associative)
    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_not()?;
        while self.peek() == Some('*') {
            self.consume();
            lhs = Expr::And(Box::new(lhs), Box::new(self.parse_not()?));
        }
        Ok(lhs)
    }

    /// Parse negation (prefix operator, right-associative)
    fn parse_not(&mut self) -> Result<Expr, String> {
        if self.peek() == Some('~') {
            self.consume();
            Ok(Expr::Not(Box::new(self.parse_not()?)))
        } else {
            self.parse_primary()
        }
    }

    /// Parse primary expressions: variables or parenthesized expressions
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.consume() {
            Some('(') => {
                let expr = self.parse_expr()?;
                if self.consume() == Some(')') { Ok(expr) } 
                else { Err("Expected closing parenthesis".to_string()) }
            }
            Some(c) if c.is_alphanumeric() => {
                // Read multi-character variable names
                let mut s = c.to_string();
                while let Some(nc) = self.peek() {
                    if nc.is_alphanumeric() { self.consume(); s.push(nc); } 
                    else { break; }
                }
                Ok(Expr::Var(s))
            }
            Some(c) => Err(format!("Unexpected character: {}", c)),
            None => Err("Unexpected end of input".to_string()),
        }
    }
}

// ============================================================================
// LOGIC ENGINE
// ============================================================================

/// Helper function to negate an expression (handles double negation elimination)
fn negate(e: &Expr) -> Expr {
    match e {
        Expr::Not(inner) => *inner.clone(),  // ~~A becomes A
        _ => Expr::Not(Box::new(e.clone())), // A becomes ~A
    }
}

/// Main solver: attempts to prove or disprove the deduction from given premises
/// Uses forward chaining with inference rules to derive new facts until:
///   - The target is found (Valid)
///   - The negation of target is found (Invalid)
///   - No more facts can be derived (Insufficient Information)
fn solve(premises: Vec<Expr>, deduction: Expr) {
    let mut facts: Vec<Fact> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();  // Prevent duplicate facts

    // Initialize knowledge base with premises
    for (i, p) in premises.into_iter().enumerate() {
        seen.insert(format!("{:?}", p));
        facts.push(Fact { expr: p, source: Source::Premise, id: i });
    }

    let target = deduction;
    let neg_target = negate(&target);
    let mut result = (None, "Insufficient Information");
    let mut idx = 0;

    // Forward chaining: process each fact and derive new ones
    while idx < facts.len() && facts.len() < 10000 {
        let cur = facts[idx].clone();

        // Check if we've reached our goal
        if cur.expr == target { result = (Some(idx), "Valid"); break; }
        if cur.expr == neg_target { result = (Some(idx), "Invalid"); break; }

        let mut new = Vec::new();
        
        // Apply single-fact inference rules
        // Simplification: (A * B) => A, (A * B) => B
        if let Expr::And(a, b) = &cur.expr {
            new.push((*a.clone(), "Simplification".to_string(), vec![cur.id]));
            new.push((*b.clone(), "Simplification".to_string(), vec![cur.id]));
        }
        
        // Apply two-fact inference rules with all other facts
        for j in 0..facts.len() {
            if j == idx { continue; }
            let other = &facts[j];
            // Try both orderings since some rules are order-dependent
            apply_rules(&cur, other, &facts, &mut new);
            apply_rules(other, &cur, &facts, &mut new);
        }

        // Add newly derived facts to knowledge base
        for (expr, rule, parents) in new {
            let key = format!("{:?}", expr);
            if !seen.contains(&key) {
                seen.insert(key);
                let nid = facts.len();
                facts.push(Fact { 
                    expr: expr.clone(), 
                    source: Source::Derived { rule, parents }, 
                    id: nid 
                });
                
                // Check immediately if new fact proves our goal
                if expr == target { result = (Some(nid), "Valid"); break; }
                if expr == neg_target { result = (Some(nid), "Invalid"); break; }
            }
        }
        
        if result.0.is_some() { break; }
        idx += 1;
    }

    // Output results
    println!("Determination: {}", result.1);
    if let Some(id) = result.0 {
        println!("\nProof Trace:");
        print_proof(&facts, id);
    }
}

/// Apply all two-fact inference rules between two facts
/// Rules implemented:
///   - Modus Ponens: (A > B), A => B
///   - Modus Tollens: (A > B), ~B => ~A
///   - Hypothetical Syllogism: (A > B), (B > C) => (A > C)
///   - Disjunctive Syllogism: (A + B), ~A => B
///   - Addition: A => (A + B) when (A + B) appears in an implication
///   - Conjunction: A, B => (A * B) when (A * B) appears in an implication
fn apply_rules(p1: &Fact, p2: &Fact, all: &[Fact], out: &mut Vec<(Expr, String, Vec<usize>)>) {
    let (e1, e2) = (&p1.expr, &p2.expr);

    // Modus Ponens: If we have (A > B) and A, derive B
    if let Expr::Implies(a, b) = e1 {
        if **a == *e2 { 
            out.push((*b.clone(), "Modus Ponens".to_string(), vec![p1.id, p2.id])); 
        }
    }

    // Modus Tollens: If we have (A > B) and ~B, derive ~A
    if let Expr::Implies(a, b) = e1 {
        if negate(b) == *e2 { 
            out.push((negate(a), "Modus Tollens".to_string(), vec![p1.id, p2.id])); 
        }
    }

    // Hypothetical Syllogism: If we have (A > B) and (B > C), derive (A > C)
    if let (Expr::Implies(a, b), Expr::Implies(c, d)) = (e1, e2) {
        if **b == **c { 
            out.push((Expr::Implies(a.clone(), d.clone()), "Hypothetical Syllogism".to_string(), vec![p1.id, p2.id])); 
        }
    }

    // Disjunctive Syllogism: If we have (A + B) and ~A, derive B
    if let Expr::Or(a, b) = e1 {
        if negate(a) == *e2 { 
            out.push((*b.clone(), "Disjunctive Syllogism".to_string(), vec![p1.id, p2.id])); 
        }
        else if negate(b) == *e2 { 
            out.push((*a.clone(), "Disjunctive Syllogism".to_string(), vec![p1.id, p2.id])); 
        }
    }

    // Addition: If we have A and see an implication with (A + B) as antecedent,
    // derive (A + B) so Modus Ponens can fire
    if let Expr::Implies(ant, _) = e2 {
        if let Expr::Or(l, r) = ant.as_ref() {
            if **l == *e1 || **r == *e1 { 
                out.push((ant.as_ref().clone(), "Addition".to_string(), vec![p1.id])); 
            }
        }
    }

    // Conjunction: If we have A and B, derive (A * B) but ONLY if it matches
    // an implication's antecedent to avoid exponential explosion
    let conj = Expr::And(Box::new(e1.clone()), Box::new(e2.clone()));
    let conj_rev = Expr::And(Box::new(e2.clone()), Box::new(e1.clone()));
    
    for f in all {
        if let Expr::Implies(ant, _) = &f.expr {
            if let Expr::And(_, _) = ant.as_ref() {
                if **ant == conj || **ant == conj_rev {
                    out.push((ant.as_ref().clone(), "Conjunction".to_string(), vec![p1.id, p2.id]));
                    return; // Only generate once
                }
            }
        }
    }
}

// ============================================================================
// PROOF TRACING
// ============================================================================

/// Recursively collect all facts that a given fact depends on (in dependency order)
fn collect_deps(facts: &[Fact], id: usize, list: &mut Vec<usize>) {
    if list.contains(&id) { return; }
    if let Source::Derived { parents, .. } = &facts[id].source {
        for &p in parents { collect_deps(facts, p, list); }
    }
    list.push(id);
}

/// Print a linear proof trace showing how the target was derived
fn print_proof(facts: &[Fact], target_id: usize) {
    let mut steps = Vec::new();
    collect_deps(facts, target_id, &mut steps);

    for (n, &idx) in steps.iter().enumerate() {
        let f = &facts[idx];
        match &f.source {
            Source::Premise => {
                println!("Step {}: {} [Premise]", n + 1, f.expr);
            }
            Source::Derived { rule, parents } => {
                // Map parent fact IDs to their step numbers in the proof
                let ps: Vec<_> = parents.iter()
                    .map(|&id| format!("Step {}", steps.iter().position(|&x| x == id).unwrap() + 1))
                    .collect();
                println!("Step {}: {} [Derived from {} using {}]", 
                    n + 1, f.expr, ps.join(", "), rule);
            }
        }
    }
}

// ============================================================================
// MAIN ENTRY POINT
// ============================================================================

fn main() {
    let stdin = io::stdin();
    let mut premises = Vec::new();
    let mut deduction = None;

    println!("Enter premises (one per line). Format: ~ (Not), * (And), + (Or), > (Implies).");
    println!("End input with 'R' followed by deduction (e.g. 'R j').");

    // Read premises and deduction from stdin
    for line in stdin.lock().lines().map_while(Result::ok) {
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }

        if let Some(content) = trimmed.strip_prefix('R') {
            // Line starting with 'R' indicates the deduction to prove
            if let Ok(e) = Parser::new(content.trim()).parse_expr() {
                deduction = Some(e);
                break;
            }
        } else if let Ok(e) = Parser::new(trimmed).parse_expr() {
            premises.push(e);
        }
    }

    if let Some(d) = deduction {
        solve(premises, d);
    } else {
        println!("No deduction found.");
    }
}