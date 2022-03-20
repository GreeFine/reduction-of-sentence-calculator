#[derive(Clone, Default)]
pub struct Options {
    pub rps: bool,
    pub crp: bool,
}

#[allow(dead_code)]
pub enum OptionsName {
    Rps,
    Crp,
}
