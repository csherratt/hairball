extern crate hairball;

use hairball::{HairballBuilder, LocalEntity};

#[test]
fn build_file() {
	let mut hairball = HairballBuilder::new();

	for i in 0..4 {
		hairball.add_entity(LocalEntity::anonymous());
	}

	hairball.add_entity(LocalEntity::named("Kitten".to_string()));

    let mut file = std::fs::File::create("test.hairball").unwrap();
    hairball.write(&mut file);


}