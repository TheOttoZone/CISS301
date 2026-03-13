use peg::*;

peg::parser!{
  grammar iptables_parser() for str{
    rule _() = quiet!{[' ']+} // whitespace rule for simplicity later
    
    rule alphanumeric() = quiet!{['a'..='z' | 'A'..='Z' | '0'..='9']+}

    rule number() = quiet!{['0'..='9']}

    // tried to limit numbers to 3 digits which works, but errors unhelpfully
    rule ipaddr() = number()*<1,3> "." number()*<1,3> "." number()*<1,3> "." number()*<1,3> ("/" number()*<1,2>)? 

    rule iptables() -> &'input str
      = $("iptables")

    rule append() -> (&'input str, &'input str)
      = flag:$("-A") _ io:$("INPUT" / "OUTPUT" / "FORWARD") {(flag, io)}
      
    rule jump() -> (&'input str, &'input str)
      = flag:$("-j") _ target:$("DROP" / "ACCEPT" / "REJECT" / "LOG") {(flag, target)}

    rule interface() -> (&'input str, &'input str)
      = flag:$("-i") _ int:$(alphanumeric() / expected!("a valid interface!")) {(flag, int)}
  
    rule source() -> (&'input str, &'input str)
      = flag:$("-s") _ ip:$(ipaddr() / expected!("a valid IP address!")) {(flag, ip)}

    rule dest() -> (&'input str, &'input str)
      = flag:$("-d") _ ip:$(ipaddr() / expected!("a valid IP address!")) {(flag, ip)}

    rule match() -> (&'input str, &'input str, &'input str, &'input str)
      = flag:$("-m") _ module:$("state") _ flag2:$("--state") _ flag3:$("NEW" / "ESTABLISHED" / "RELATED" / "INVALID") {(flag, module, flag2, flag3)}

    rule protocol() -> (&'input str, &'input str)
      = flag:$("-p") _ prot:$("udplite" / "icmpv6" / "tcp" / "udp" / "icmp" / "esp" / "ah" / "sctp" / "mh") {(flag, prot)}

    rule dport() -> (&'input str, &'input str)
      = flag:$("--dport") _ port:$(number()*<1,5> / expected!("a valid port!")) {(flag, port)}

    rule icmp() -> (&'input str, &'input str)
      = flag:$("--icmp-type") _ tipe:$("0" / "8" / "echo-reply" / "echo-request") {(flag, tipe)}

    rule param() = interface() / source() / dest() / match() / protocol() / dport() / icmp() // rule for testing optional arguments
    
    pub rule full() -> bool
      = iptables() _ append() (_ param())* ( _ jump() / jump() ) ![_] {true}
  } 
}

pub fn main() {
  let required_rules = vec![
    // required rules
    "iptables -A INPUT -s 1.2.3.4 -j DROP", // 1
    "iptables -A INPUT -s 192.168.0.0/24 -j DROP", // 2 
    "iptables -A INPUT -i eth1 -s 192.168.0.0/24 -j DROP", // 3
    "iptables -A INPUT -i eth1 -p tcp --dport 80 -j DROP", // 4
    "iptables -A OUTPUT -d 192.168.1.0/24 -j DROP", // 5
    "iptables -A OUTPUT -p tcp -d 69.171.224.0/19 -j DROP", // 6
    "iptables -A INPUT -p icmp --icmp-type echo-request -j DROP", // 7
    "iptables -A INPUT -m state --state NEW -p tcp --dport 25 -j ACCEPT", // 8
    "iptables -A OUTPUT -p tcp -d 192.168.40.0/24 --dport 22 -j ACCEPT", // 9
  ]; 

  let working_rules = vec![
    // rules that will succeed to show variations that arent in required rules
    "iptables -A FORWARD -s 1.2.3.4 -j REJECT", // 10
    "iptables -A INPUT -m state --state ESTABLISHED -p udp --dport 25 -j LOG", // 11
    "iptables -A INPUT -m state --state RELATED -p udplite --dport 25 -j DROP", // 12
    "iptables -A INPUT -m state --state INVALID -p esp --dport 25 -j LOG", // 13
    "iptables -A INPUT -p icmpv6 --icmp-type echo-reply -j DROP", // 14
    "iptables -A INPUT -p ah --icmp-type 0 -j DROP", // 15
    "iptables -A INPUT -p sctp --icmp-type 8 -j DROP", // 16
    "iptables -A INPUT -p mh --icmp-type 8 -j DROP", // 17
  ];

  let error_rules = vec![
    // rules that will error, meant to test rules
    "ipeetables -A INPUT -s 1.2.3.4 -j DROP", // 18 tests iptables
    "iptables -s 1.2.3.4 -j DROP", // 19 tests append()
    "iptables -A FOREWARD -s 192.168.0.0/24 -j DROP", // 20 tests append() 2
    "iptables -A FORWARD -i *eeth1_+ -s 192.168.0.0/24 -j DROP", // 21 tests interface()
    "iptables -A INPUT -i eth1 -p poo --dport 80 -j DROP", // 22 tests protocol()
    "iptables -A OUTPUT -d 192.168.1.0/24", // 23 tests jump()
    "iptables -A INPUT -i eth1 -p tcp --dport blah -j DROP", // 24 tests dport()
    "iptables -A INPUT -p icmp --icmp-type poo -j DROP", // 25 tests icmp()
    "iptables -A INPUT -s 2555.2.3.4 -j DROP", // 26 tests ip grabbing
    "iptables -A INPUT -d dsfhdsjkfhj -j DROP", // 27 tests ip grabbing again
    "iptables -A INPUT -m notstate --state INVALID -p esp --dport 25 -j LOG", // 28 tests match()
    "iptables -A INPUT -m state -p esp --dport 25 -j LOG", // 29 also tests match()
    "iptables -A INPUT -m state --state abababababa -p esp --dport 25 -j LOG" // 30 also tests match()
  ];

  let mut count = 1; // simple number to display that allows me to match output to which line here
  println!("Required Rules:");
  parse(required_rules, &mut count);
  println!("Other working rules:");
  parse(working_rules, &mut count);
  println!("Rules to test error checking:");
  parse(error_rules, &mut count);
}

fn parse(rules: Vec<&str>, count: &mut i32) {
    for i in rules {
        print!("{count}");
        match iptables_parser::full(i) {
            Ok(_) => println!("\t✅ \"{}\" is a valid command!", i),
            Err(e) => println!("\t❌  Error at {}, expected {}", e.location, e.expected),
        }
        *count += 1;
    }
}