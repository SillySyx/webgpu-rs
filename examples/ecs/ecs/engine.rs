pub trait Engine: 'static {
    fn new() -> Self where Self: Sized;

    fn update(&mut self);

    fn frame(&mut self);
}