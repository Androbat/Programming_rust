mod spores {
    use cells::{Cell, Gene};
    /// A cell made by an adult fern. It disperses on the wind as part of
    /// the fern file cycle. 
    pub struct Spore { }
    pub fn produce_spore(factory: &mut Sorangium) -> Spore {}

    // Extract the genes in a particular spore.
    pub (crate) fn genes(spore: &Spore) -> Vec<Gene> {}
    fn recombine(parent: &mut Cell) {}

}