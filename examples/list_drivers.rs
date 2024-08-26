use j2534::drivers::list as list_j2534_drivers;

fn main() {
    println!("Available J2534 DLLs:");
    for (i, dll) in list_j2534_drivers().unwrap().iter().enumerate() {
        println!("  {}) {}: {} | {}", i+1, dll.name, dll.path, dll.protocols);
    }
}