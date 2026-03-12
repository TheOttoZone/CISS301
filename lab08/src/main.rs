use peg::*;

peg::parser!{
  grammar iptables_parser() for str{
    rule _() = quiet!{[' ']+}
    
    rule alphanumeric() = ['a'..='z' | 'A'..='Z' | '0'..='9']+

    rule number() = ['0'..='9']+

    rule ipaddr() = number() "." number() "." number() "." number() ("/" number())?

    rule iptables() -> &'input str
      = $("iptables")

    rule append() -> (&'input str, &'input str)
      = flag:$("-A") _ io:$("INPUT" / "OUTPUT" / "FORWARD" ) {(flag, io)}
      / expected!("Not a valid chain!")

    rule target() -> (&'input str, &'input str)
      = flag:$("-j") _ da:$("DROP" / "ACCEPT" / "REJECT" / "LOG") {(flag, da)}
      / expected!("Not a valid target!")

    rule interface() -> (&'input str, &'input str)
      = flag:$("-i") _ int:$(alphanumeric()) {(flag, int)}
      / expected!("Not a valid interface!")

    rule source() -> (&'input str, &'input str)
      = flag:$("-s") _ ip:$(ipaddr()) {(flag, ip)}
      / expected!("Not a valid IP address!")

    rule dest() -> (&'input str, &'input str)
      = flag:$("-d") _ ip:$(ipaddr()) {(flag, ip)}
      / expected!("Not a valid IP address!")

    rule param() = interface() / source() / dest()
    
    pub rule full() -> bool
      = iptables() _ append() (_ param())* _ target() !(_) {true} 
  } 
}

pub fn main() {
    let iptables_rules = vec![
        "iptables -A INPUT -s 1.2.3.4 -j DROP",
        "iptables -A INPUT -s 192.168.0.0/24 -j DROP",
        "iptables -A INPUT -i eth1 -s 192.168.0.0/24 -j DROP",
        "iptables -A INPUT -i eth1 -p tcp --dport 80 -j DROP",
        "iptables -A OUTPUT -d 192.168.1.0/24 -j DROP",
        "iptables -A OUTPUT -p tcp -d 69.171.224.0/19 -j DROP",
        "iptables -A INPUT -p icmp --icmp-type echo-request -j DROP",
        "iptables -A INPUT -m state --state NEW -p tcp --dport 25 -j ACCEPT",
        "iptables -A OUTPUT -p tcp -d 192.168.40.0/24 --dport 22 -j ACCEPT"
    ];

    for i in iptables_rules {
      match iptables_parser::full(i) {
        Ok(_) => println!("✅ {} is a valid command!", i),
        Err(e) => println!("❌  Error at {}", e.expected),
      }
    }
}