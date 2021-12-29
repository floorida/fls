use colored::Color;
// pub struct IconPack {
//     pub rust: char,
//     pub python: char,
//     pub js: char,
//     pub ts: char,
//     pub c: char,
//     pub cpp: char,
//     pub java: char,
//     pub html: char,
//     pub php: char,
//     pub go: char,
//     pub elixir: char,
//     pub swift: char,
//     pub bash: char,
//     pub folder: char,
//     pub video: char,
//     pub text: char,
//     pub perl: char,
//     pub ruby: char,
// }

// pub const ICONS: IconPack = IconPack {
//     rust: '',
//     python: '',
//     js: '',
//     ts: 'ﯤ',
//     c: '',
//     cpp: '',
//     java: '',
//     html: '',
//     php: '',
//     go: '',
//     elixir: '',
//     swift: '',
//     bash: '',
//     folder: '',
//     video: '',
//     text: '',
//     perl: '',
//     ruby: ''
// };

pub enum Extensions {
    Mp4,
    Mkv,
    Mov,
    Sh,
    Rs,
    Java,
    Py,
    Js,
    Cpp,
    Clang,
    Ts,
    Html,
    Css,
    Go,
    R,
    Asm,
    Unknown,
}

pub fn parse_file_type(name: &str) -> Option<Extensions> {
    Some(match name {
        // all the extension matching will be happening here
        "mp4" => Extensions::Mp4,  
        "mkv" => Extensions::Mkv,
        "mov" => Extensions::Mov,
        "sh" => Extensions::Sh,
        "rs" => Extensions::Rs,
        "java" => Extensions::Java,
        "py" => Extensions::Py,
        "js" => Extensions::Js,
        "cpp" => Extensions::Cpp,
        "c" => Extensions::Clang,
        "ts" => Extensions::Ts,
        "html" => Extensions::Html,
        "css" => Extensions::Css,
        "go" => Extensions::Go,
        "r" => Extensions::R,
        "asm" => Extensions::Asm,
        _ => Extensions::Unknown,
    })
}

pub fn get_color(ext: Extensions) -> Option<Color> {
    Some(match ext {
        Extensions::Mkv | Extensions::Mp4 | Extensions::Mov => Color::Magenta,
        Extensions::Sh | Extensions::Rs | Extensions::Java | Extensions::Py | Extensions::Js | Extensions::Cpp | Extensions::Clang | Extensions::Ts | Extensions::Html | Extensions::Css | Extensions::Go | Extensions::R | Extensions::Asm => Color::Yellow,
        Extensions::Unknown => Color::White,
    })
}