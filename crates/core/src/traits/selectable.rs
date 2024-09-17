//

pub trait Selectable {
    fn matches(&self, selection: &Selection) -> bool;
    // Other methods as needed
}
