pub struct PanelDims {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

pub struct Panel<Kind> {
    pub(super) kind: Kind,
}
