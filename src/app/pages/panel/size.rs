use cosmic_panel_config::PanelSize;

#[rustfmt::skip]
const PANEL_SIZES: &[&str] = &[
    // 16, 20, 24, 28, 32
    "XS-4", "XS-3", "XS-2", "XS-1", "XS",
    // 36, 40, 44, 48, 52
    "S-1", "S", "S+1", "S+2", "S+3",
    // 56, 60
    "M", "M+1",
    // 64, 68, 72, 76
    "L", "L+1", "L+2", "L+3",
    // 80, 84, 88, 92, 96, 100, 104, 108, 112
    "XL-4", "XL-3", "XL-2", "XL-1", "XL", "XL+1", "XL+2", "XL+3", "XL+4",
];

pub fn name(size: PanelSize) -> &'static str {
    let custom = match size {
        PanelSize::XS => return "XS",
        PanelSize::S => return "S",
        PanelSize::M => return "M",
        PanelSize::L => return "L",
        PanelSize::XL => return "XL",
        PanelSize::Custom(custom) => custom,
    } as usize;
    let idx = (custom.clamp(16, 112) - 16) / 4;
    PANEL_SIZES[idx]
}

pub(crate) fn to_u32(size: PanelSize) -> u32 {
    match size {
        PanelSize::XS => 32,
        PanelSize::S => 40,
        PanelSize::M => 56,
        PanelSize::L => 64,
        PanelSize::XL => 96,
        PanelSize::Custom(custom) => custom.clamp(16, 112) / 4 * 4,
    }
}
