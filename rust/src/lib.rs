use godot::prelude::*;
use godot::classes::INode;

struct GameExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GameExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct HelloWorld {
	base: Base<Node>
}

#[godot_api]
impl INode for HelloWorld {
	fn init(base: Base<Node>) -> Self {
		godot_print!("Hello, world!");
		Self {
			base
		}
	}
}
