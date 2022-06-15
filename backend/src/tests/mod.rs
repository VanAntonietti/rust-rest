use crate::hello_world;

#[test]
fn test_hello_world() {
	let x = "Tuturu";
	let y = hello_world(x);
	assert_eq!(x, y);
}
