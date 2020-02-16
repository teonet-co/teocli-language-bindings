#[link(name = "teocli", kind = "static")]
extern "C" {
    fn teoLNullInit() -> ();
}

fn main() {
    let host_name = "teocli_rust";
    let ip = "5.63.158.100";
    let port = 9010;
    let peer_name = "ps-server";
    let msg = "Hello from rust";
    let tcp_f = 1;
unsafe {
    teoLNullInit();
}
    println!("{}", msg);
}
