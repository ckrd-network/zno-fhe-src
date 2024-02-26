use fake::faker::name::en::Name;
use fake::Fake;

#[test]
fn test_fake_data() {
    let name: String = Name().fake();
    println!("Generated name: {}", name);
    assert!(!name.is_empty());
}
