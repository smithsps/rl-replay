// Body Parsing

use nom::*;


#[derive(Debug)]
pub struct Keyframe {
    time: f32,
    frame: i32,
    file_pos: i32
}

named!(pub keyframe_array <Vec<Keyframe>>,
    do_parse!(
        keyframes: length_count!(le_u32, do_parse!(
            time: le_f32 >>
            frame: le_i32 >>
            file_pos: le_i32 >>

            (Keyframe {
                time: time,
                frame: frame,
                file_pos: file_pos
            })
        )) >>
        (keyframes)
    )
);

#[derive(Debug)]
pub struct NetworkStream<> {
    length: u32
}

named!(pub get_netstream <NetworkStream>,
    do_parse!(
        length: le_u32 >>
        take!(length) >>
        (NetworkStream {
            length: length
        })
    )
);


#[derive(Debug)]
pub struct DebugString<'a> {
    frame: i32,
    username: &'a str,
    text: &'a str
}

named!(pub debug_strings_array <Vec<DebugString>>,
    do_parse!(
        debug_strings: length_count!(le_u32, do_parse!(
            frame: le_i32 >>
            username: raw_string >>
            text: raw_string >>

            (DebugString {
                frame: frame,
                username: username,
                text: text
            })
        )) >>
        (debug_strings)
    )
);

#[derive(Debug)]
pub struct TickMark<'a> {
    tick_type: &'a str,
    frame: i32
}

named!(pub tick_marks_array <Vec<TickMark>>,
    do_parse!(
        tick_marks: length_count!(le_u32, do_parse!(
            tick_type: raw_string >>
            frame: le_i32 >>
            (TickMark {
                tick_type: tick_type,
                frame: frame
            })
        )) >>
        (tick_marks)
    )
);

#[derive(Debug)]
pub struct ClassIndex<'a> {
    class: &'a str,
    index: i32
}

named!(pub class_index_array <Vec<ClassIndex>>,
    do_parse!(
        class_indexs: length_count!(le_u32, do_parse!(
            class: raw_string >>
            index: le_i32 >>
            (ClassIndex {
                class: class,
                index: index
            })
        )) >>
        (class_indexs)
    )
);

#[derive(Debug)]
pub struct ClassNetCache<> {
    length: u32
}

named!(pub get_class_net_cache <ClassNetCache>,
    do_parse!(
        length: le_u32 >>
        (ClassNetCache {
            length: length
        })
    )
);

named!(pub raw_string<&str>,
    do_parse!(
        length: le_u32 >>
        string: take_str!(length - 1) >>
        null_terminator: take!(1) >>
        (string)
    )
);

#[derive(Debug)]
pub struct ReplayBody<'a> {
    levels: Vec<&'a str>,
    keyframes: Vec<Keyframe>,
    network_stream: NetworkStream,
    debug_strings: Vec<DebugString<'a>>,
    tick_marks: Vec<TickMark<'a>>,
    packages: Vec<&'a str>,
    objects: Vec<&'a str>,
    names: Vec<&'a str>,
    class_indexes: Vec<ClassIndex<'a>>,
    class_net_cache: ClassNetCache,
}

named!(pub get_body<(ReplayBody)>,
    do_parse!(
        levels: length_count!(le_u32, raw_string) >>
        keyframes: keyframe_array >>
        network_stream: get_netstream >>
        debug_strings: debug_strings_array >>
        tick_marks: tick_marks_array >>
        packages: length_count!(le_u32, raw_string) >>
        objects: length_count!(le_u32, raw_string) >>
        names: length_count!(le_u32, raw_string) >>
        class_indexes: class_index_array >>
        class_net_cache: get_class_net_cache >>

        (ReplayBody {
            levels: levels,
            keyframes: keyframes,
            network_stream: network_stream,
            debug_strings: debug_strings,
            tick_marks: tick_marks,
            packages: packages,
            objects: objects,
            names: names,
            class_indexes: class_indexes,
            class_net_cache: class_net_cache
        })
    )

);