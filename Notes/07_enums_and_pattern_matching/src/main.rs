// Where structs give you a way of grouping together related fields and data, like a Rectangle with its width and height, enums give you a way of saying a value is one of a possible set of values. For example, we may want to say that Rectangle is one of a set of possible shapes that also includes Circle and Triangle. To do this, Rust allows us to encode these possibilities as an enum.

// For instance, Any IP address can be either a version four or a version six address, but not both at the same time. That property of IP addresses makes the enum data structure appropriate because an enum value can only be one of its variants. Both version four and version six addresses are still fundamentally IP addresses, so they should be treated as the same type when the code is handling situations that apply to any kind of IP address.

#[derive(Debug)]
enum IpAddressKind {
    // you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example. You can even include another enum!
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddressKind {
    fn echo(&self) -> &Self {
        // insert method logic here
        self
    }
}

fn main() {
    // The variants of the enum are both namespaced under its identifier, and we use the double colon to separate the two. This is useful since `IpAddressKind::v4` and `IpAddressKind::v6` are of the same type: `IpAddressKind`.
    let ipv4 = IpAddressKind::V4(127, 0, 0, 1); // IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type. We automatically get this constructor function defined as a result of defining the enum.
    let ipv6 = IpAddressKind::V6;

    let loopback = ipv6(String::from("::1"));

    route(ipv4);

    let echo_test_v4 = IpAddressKind::V4(192, 168, 0, 1);

    println!("{:?}", echo_test_v4.echo());
}

fn route(ip_kind: IpAddressKind) {
    // foobar
}

// Rust doesn't have null. However, the concept that null tries to express is presented in the form of the enum `Option <T>`:
// enum Option<T> {
//     None,
//     Some(T),
// }

// The Option<T> enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly. Its variants are also included in the prelude: you can use Some and None directly without the Option:: prefix. The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>.

// But what IS <T>?  Well, it's a generic type parameter (reminds me of generics in TS so that's a diversity win if i've ever seen one).

// For now, all you need to know is that <T> means that the Some variant of the Option enum can hold one piece of data of any type, and that each concrete type that gets used in place of T makes the overall Option<T> type a different type. Basically it's like having the Protean ability.

fn generics_examples() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None; // For absent_number, Rust requires us to annotate the overall Option type: the compiler can’t infer the type that the corresponding Some variant will hold by looking only at a None value. Here, we tell Rust that we mean for absent_number to be of type Option<i32>.
}
