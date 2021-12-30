pub struct Mcu {
    pub family: McuFamily
    //TODO?
}

/// Only families supported by the compiler are enlisted, other families fall into `Unsupported` category.
#[non_exhaustive]
pub enum McuFamily{
    F4,
    Unsupported,
}