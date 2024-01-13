// NOTE: enum stands fo enumerator

enum DiskType {
    SSD, // These fields are called variants
    HDD,
}

#[derive(Debug)]

enum DiskSize {

    KB(u32),
    MB(u32),
    GB(u32),

}

fn main() {
    
    // NOTE: How to use an enum
    let disk_type = DiskType::SSD;
    
    match disk_type {
        DiskType::SSD => println!("Disk type is SSD"),
        DiskType::HDD => println!("Disk type is HDD"),

    }

}
