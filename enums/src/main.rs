#[derive(Debug)]
enum RelexProduct {
    Store(String),
    Plan(String),
    Workforece(String)
}

impl RelexProduct {
    fn print() {
        println!("enum information");
    }
}

struct Feature {
    product: RelexProduct,
    feature: String
}

fn main() {
    println!("Hello, enums!");
    let feature = RelexProduct::Store(String::from("v3.2"));
    println!("{:#?}", feature);

    // let v3Feature = Feature {
    //     product: RelexProduct::Store,
    //     feature: String::from("Floor Planning")
    // };

}
