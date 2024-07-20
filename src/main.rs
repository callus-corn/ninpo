use ninpo::connect::Connect;

fn main() {
    println!("Hello, world!");
    if let Ok(mut conn) = Connect::open(Some("qemu:///session")) {
      assert_eq!(Ok(0), conn.close());
    }
    println!("Hello, world!");
}
