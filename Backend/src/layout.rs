use crate::qr_json::Layout;
use fast_qr::convert::Shape;
use fast_qr::ModuleType;
pub fn choose_layout(layout: Layout) -> Shape {
    match layout {
        Layout::Squares => Shape::Command(|y, x, cell| match cell.module_type() {
            ModuleType::FinderPattern | ModuleType::Alignment => String::new(),
            _ => Shape::Square(y, x, cell),
        }),
        Layout::RoundedSquares => Shape::Command(|y, x, cell| match cell.module_type() {
            ModuleType::FinderPattern | ModuleType::Alignment => String::new(),
            _ => Shape::RoundedSquare(y, x, cell),
        }),
        Layout::Circles => Shape::Command(|y, x, cell| match cell.module_type() {
            ModuleType::FinderPattern | ModuleType::Alignment => String::new(),
            _ => Shape::Circle(y, x, cell),
        }),
        Layout::Rectangles => Shape::Command(|y, x, cell| match cell.module_type() {
            ModuleType::FinderPattern | ModuleType::Alignment => String::new(),
            _ => Shape::Vertical(y, x, cell),
        }),
    }
}
