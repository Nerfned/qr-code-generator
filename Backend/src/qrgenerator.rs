use crate::{layout::choose_layout, qr_json::Layout, version::generate_version};
use fast_qr::convert::svg::SvgBuilder;
use fast_qr::{
    convert::{Builder, Shape},
    ModuleType, QRBuilder, ECL,
};

pub fn qr_generator(input: String, colour: &str, layout: Layout, logo: Option<String>) -> String {
    let mut input_len = input.len();
    if logo.is_some() {
        input_len = (input_len as f64 * (1.0 + 1.0 / 9.0)) as usize;
    }

    let qrcode = QRBuilder::new(input)
        .ecl(ECL::H)
        .version(generate_version(input_len))
        .build()
        .unwrap();

    match logo {
        Some(logo) => SvgBuilder::default()
            .shape(choose_layout(layout))
            .shape(Shape::Command(|y, x, cell| match cell.module_type() {
                ModuleType::FinderPattern | ModuleType::Alignment => Shape::Square(y, x, cell),
                _ => String::new(),
            }))
            .image_background_color("#FFFFFF00")
            .image(logo)
            .background_color("#FFFFFF00")
            .module_color(colour)
            .to_str(&qrcode),
        None => SvgBuilder::default()
            .shape(choose_layout(layout))
            .shape(Shape::Command(|y, x, cell| match cell.module_type() {
                ModuleType::FinderPattern | ModuleType::Alignment => Shape::Square(y, x, cell),
                _ => String::new(),
            }))
            .background_color("#FFFFFF00")
            .module_color(colour)
            .to_str(&qrcode),
    }
}
