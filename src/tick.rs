use wasm4::rt::Runtime;

pub trait Tick<R: Runtime> {
    fn update(self, rt: &mut R) -> Self;
}
