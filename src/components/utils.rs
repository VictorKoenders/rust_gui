use vecmath::Vector4;

pub type SizeType = i16;

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Unknown,
    Pixels(SizeType),
    Percentage(SizeType),
}

impl Size {
    pub fn get_pixel_position(self, min_value: SizeType, max_value: SizeType) -> SizeType {
        match self {
            Size::Unknown => min_value,
            Size::Pixels(p) => min_value + p as SizeType,
            Size::Percentage(p) => ((max_value - min_value) * p as SizeType / 100) + min_value,
        }
    }

    pub fn get_pixel_position_reversed(self, min_value: SizeType, max_value: SizeType) -> SizeType {
        match self {
            Size::Unknown => max_value,
            Size::Pixels(p) => max_value - p as SizeType,
            Size::Percentage(p) => max_value - ((max_value - min_value) * p as SizeType / 100),
        }
    }

    pub fn is_unknown(self) -> bool {
        match self {
            Size::Unknown => true,
            _ => false,
        }
    }
    pub fn is_known(self) -> bool {
        match self {
            Size::Unknown => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Unknown,
    Predefined(PredefinedColor),
    RGB(f32, f32, f32),
    RGBA(f32, f32, f32, f32),
}

impl Color {
    pub fn to_vector(&self) -> Vector4<f32> {
        match *self {
            Color::Unknown => [1f32, 1f32, 1f32, 1f32],
            Color::Predefined(x) => {
                let x = x as i32;
                [((x >> 16) & 0xFF) as f32 / 255f32,
                 ((x >> 8) & 0xFF) as f32 / 255f32,
                 (x & 0xFF) as f32 / 255f32,
                 1f32]
            }
            Color::RGB(r, g, b) => [r, g, b, 1f32],
            Color::RGBA(r, g, b, a) => [r, g, b, a],
        }
    }
}

impl Into<Color> for PredefinedColor {
    fn into(self) -> Color {
        Color::Predefined(self)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum PredefinedColor {
    pink = 0xffc0cb,
    lightpink = 0xffb6c1,
    hotpink = 0xff69b4,
    deeppink = 0xff1493,
    palevioletred = 0xdb7093,
    mediumvioletred = 0xc71585,
    lightsalmon = 0xffa07a,
    salmon = 0xfa8072,
    darksalmon = 0xe9967a,
    lightcoral = 0xf08080,
    indianred = 0xcd5c5c,
    crimson = 0xdc143c,
    firebrick = 0xb22222,
    darkred = 0x8b0000,
    red = 0xff0000,
    orangered = 0xff4500,
    tomato = 0xff6347,
    coral = 0xff7f50,
    darkorange = 0xff8c00,
    orange = 0xffa500,
    yellow = 0xffff00,
    lightyellow = 0xffffe0,
    lemonchiffon = 0xfffacd,
    lightgoldenrodyellow = 0xfafad2,
    papayawhip = 0xffefd5,
    moccasin = 0xffe4b5,
    peachpuff = 0xffdab9,
    palegoldenrod = 0xeee8aa,
    khaki = 0xf0e68c,
    darkkhaki = 0xbdb76b,
    gold = 0xffd700,
    cornsilk = 0xfff8dc,
    blanchedalmond = 0xffebcd,
    bisque = 0xffe4c4,
    navajowhite = 0xffdead,
    wheat = 0xf5deb3,
    burlywood = 0xdeb887,
    tan = 0xd2b48c,
    rosybrown = 0xbc8f8f,
    sandybrown = 0xf4a460,
    goldenrod = 0xdaa520,
    darkgoldenrod = 0xb8860b,
    peru = 0xcd853f,
    chocolate = 0xd2691e,
    saddlebrown = 0x8b4513,
    sienna = 0xa0522d,
    brown = 0xa52a2a,
    maroon = 0x800000,
    darkolivegreen = 0x556b2f,
    olive = 0x808000,
    olivedrab = 0x6b8e23,
    yellowgreen = 0x9acd32,
    limegreen = 0x32cd32,
    lime = 0x00ff00,
    lawngreen = 0x7cfc00,
    chartreuse = 0x7fff00,
    greenyellow = 0xadff2f,
    springgreen = 0x00ff7f,
    mediumspringgreen = 0x00fa9a,
    lightgreen = 0x90ee90,
    palegreen = 0x98fb98,
    darkseagreen = 0x8fbc8f,
    mediumaquamarine = 0x66cdaa,
    mediumseagreen = 0x3cb371,
    seagreen = 0x2e8b57,
    forestgreen = 0x228b22,
    green = 0x008000,
    darkgreen = 0x006400,
    aqua = 0x00ffff,
    lightcyan = 0xe0ffff,
    paleturquoise = 0xafeeee,
    aquamarine = 0x7fffd4,
    turquoise = 0x40e0d0,
    mediumturquoise = 0x48d1cc,
    darkturquoise = 0x00ced1,
    lightseagreen = 0x20b2aa,
    cadetblue = 0x5f9ea0,
    darkcyan = 0x008b8b,
    teal = 0x008080,
    lightsteelblue = 0xb0c4de,
    powderblue = 0xb0e0e6,
    lightblue = 0xadd8e6,
    skyblue = 0x87ceeb,
    lightskyblue = 0x87cefa,
    deepskyblue = 0x00bfff,
    dodgerblue = 0x1e90ff,
    cornflowerblue = 0x6495ed,
    steelblue = 0x4682b4,
    royalblue = 0x4169e1,
    blue = 0x0000ff,
    mediumblue = 0x0000cd,
    darkblue = 0x00008b,
    navy = 0x000080,
    midnightblue = 0x191970,
    lavender = 0xe6e6fa,
    thistle = 0xd8bfd8,
    plum = 0xdda0dd,
    violet = 0xee82ee,
    orchid = 0xda70d6,
    fuchsia = 0xff00ff,
    mediumorchid = 0xba55d3,
    mediumpurple = 0x9370db,
    blueviolet = 0x8a2be2,
    darkviolet = 0x9400d3,
    darkorchid = 0x9932cc,
    darkmagenta = 0x8b008b,
    purple = 0x800080,
    indigo = 0x4b0082,
    darkslateblue = 0x483d8b,
    slateblue = 0x6a5acd,
    mediumslateblue = 0x7b68ee,
    white = 0xffffff,
    snow = 0xfffafa,
    honeydew = 0xf0fff0,
    mintcream = 0xf5fffa,
    azure = 0xf0ffff,
    aliceblue = 0xf0f8ff,
    ghostwhite = 0xf8f8ff,
    whitesmoke = 0xf5f5f5,
    seashell = 0xfff5ee,
    beige = 0xf5f5dc,
    oldlace = 0xfdf5e6,
    floralwhite = 0xfffaf0,
    ivory = 0xfffff0,
    antiquewhite = 0xfaebd7,
    linen = 0xfaf0e6,
    lavenderblush = 0xfff0f5,
    mistyrose = 0xffe4e1,
    gainsboro = 0xdcdcdc,
    lightgray = 0xd3d3d3,
    silver = 0xc0c0c0,
    darkgray = 0xa9a9a9,
    gray = 0x808080,
    dimgray = 0x696969,
    lightslategray = 0x778899,
    slategray = 0x708090,
    darkslategray = 0x2f4f4f,
    black = 0x000000,
}
