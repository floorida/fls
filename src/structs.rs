use colored::Color;

// TODO: impl icons since im a lazy ass lmfao 

#[derive(Debug)]
pub struct IconPack {
    pub rust: char,
    pub python: char,
    pub js: char,
    pub ts: char,
    pub c: char,
    pub cpp: char,
    pub java: char,
    pub html: char,
    pub php: char,
    pub go: char,
    pub elixir: char,
    pub swift: char,
    pub bash: char,
    pub folder: char,
    pub media: char,
    pub text: char,
    pub perl: char,
    pub ruby: char,
    pub unknown: char,
    pub config: char,
    pub yaml: char,
    pub lua: char,
}

#[allow(dead_code)]
pub const ICONS: IconPack = IconPack {
    rust: '',
    python: '',
    js: '',
    ts: 'ﯤ',
    c: '',
    cpp: '',
    java: '',
    html: '',
    php: '',
    go: '',
    elixir: '',
    swift: '',
    bash: '',
    folder: '',
    media: '',
    text: '',
    perl: '',
    ruby: '',
    unknown: '',
    config: '',
    yaml: '',
    lua: '',
};

pub enum Extensions {
    Mp4,
    Mkv,
    Mov,
    Png,
    Jpeg,
    Gimp,
    Jpg,
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
    Asm,
    Unknown,
    Out,
    D,
    In,
    Conf,
    Yaml,
    Lua,
}

pub fn parse_file_type(name: &str) -> Option<Extensions> {
    Some(match name {
        // all the extension matching will be happening here
        "mp4" => Extensions::Mp4,  
        "mkv" => Extensions::Mkv,
        "mov" => Extensions::Mov,
        "png" => Extensions::Png,
        "jpeg" => Extensions::Jpeg,
        "gimp" => Extensions::Gimp,
        "jpg" => Extensions::Jpg,
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
        "asm" => Extensions::Asm,
        "out" => Extensions::Out,
        "in" => Extensions::In,
        "d" => Extensions::D,
        "conf" => Extensions::Conf,
        "yml" | "yaml" => Extensions::Yaml,
        "lua" => Extensions::Lua,
        _ => Extensions::Unknown,

    })
}

pub fn get_color(ext: Extensions) -> Option<Color> {
    Some(match ext {
        Extensions::Mkv | Extensions::Mp4 | Extensions::Mov | Extensions::Png | Extensions::Jpeg | Extensions::Jpg | Extensions::Gimp => Color::Magenta,
        Extensions::Rs | Extensions::Lua | Extensions::Java | Extensions::Py | Extensions::Js | Extensions::Cpp | Extensions::Clang | Extensions::Ts | Extensions::Html | Extensions::Css | Extensions::Go | Extensions::Asm => Color::Yellow,
        Extensions::D  | Extensions::Out | Extensions::In | Extensions::Sh => Color::Green,
        Extensions::Yaml | Extensions::Conf => Color::Blue,
        Extensions::Unknown => Color::White,
    })
}

pub fn get_icon(ext: Extensions) -> Option<char> {
    Some(match ext {
        Extensions::Mkv | Extensions::Mp4 | Extensions::Mov | Extensions::Png | Extensions::Jpeg | Extensions::Jpg | Extensions::Gimp => ICONS.media,
        Extensions::Rs => ICONS.rust,
        Extensions::Java => ICONS.java,
        Extensions::Py => ICONS.python,
        Extensions::Js => ICONS.js,
        Extensions::Cpp => ICONS.cpp,
        Extensions::Clang => ICONS.c,
        Extensions::Ts => ICONS.ts,
        Extensions::Html => ICONS.html, 
        Extensions::Css => ICONS.html,
        Extensions::Go => ICONS.go,
        Extensions::Lua => ICONS.lua,
        Extensions::Asm | Extensions::D | Extensions::Out | Extensions::In | Extensions::Sh => ICONS.bash,
        Extensions::Yaml | Extensions::Conf => ICONS.config,
        Extensions::Unknown => ICONS.text,
    })
}