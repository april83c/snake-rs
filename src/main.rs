fn main() {
	let mut meeeee/* :3 */ = April { to_bite: Vec::new() };
	let meanie = Cobalt;
	meeeee.to_bite.push(Box::new(meanie));

	loop {
		meeeee.process();
	}
}

trait Cutie {
	fn bite(&self) {
		println!("ow i got bited");
	}
}

struct Cobalt;
impl Cutie for Cobalt {}

struct April {
	to_bite: Vec<Box<dyn Cutie>>,
}
impl April {
	fn process(&self) {
		for cutie in &self.to_bite {
			println!("teehee");
			cutie.bite();
		}
	}
}