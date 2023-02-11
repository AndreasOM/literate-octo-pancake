use std::marker::PhantomData;

trait Fruit {}

#[derive(Default)]
struct Apple<'a> {
    value: Option<&'a mut u8>,
}

impl<'b> Fruit for Apple<'b> {}

#[derive(Default)]
struct FruitPolisher<F>
where
    F: Fruit,
{
    phantom: PhantomData<F>,
}

impl<F> FruitPolisher<F>
where
    F: Fruit,
{
    //pub fn polish(&self, _fruit: &mut F ) {
    pub fn polish(&mut self, _fruit: &mut F) {}
}

#[derive(Default)]
struct FruitBox<'a> {
    apple_value: u8,
    polisher: FruitPolisher<Apple<'a>>,
}

impl FruitBox<'_> {
    pub fn polish_all(&mut self) {
        let mut apple = Apple::default();
        apple.value = Some(&mut self.apple_value);

        let polisher = &mut self.polisher;
        polisher.polish(&mut apple);
        // drop( apple );
    }

    pub fn update<'a>(&'a mut self) {
        self.polish_all();
    }
}

fn main() {
    println!("Hello, FruitBox!");
    let mut fruit_box = FruitBox::default();
    fruit_box.update();
}
