#[derive(Clone, Default)]
pub struct Options {
    pub rps: bool,
    pub crp: bool,
}

pub enum OptionsName {
    Rps,
    Crp,
}
