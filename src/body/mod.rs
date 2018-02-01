// Body Parsing

use nom::*;
use std::collections::BTreeMap;

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
pub struct RawNetworkStream<'a> {
    length: u32,
    bytes: &'a [u8]
}

named!(pub get_raw_netstream <RawNetworkStream>,
    do_parse!(
        length: le_u32 >>
        bytes: take!(length) >>
        (RawNetworkStream {
            length: length,
            bytes: bytes
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


named!(pub class_index_array <BTreeMap<i32, String>>,
    do_parse!(
        // Apply tuple!(raw_string, le_i32), length number of times.
        // Then insert these into a created BTreeMap
        length: le_u32 >>
        classes: fold_many_m_n!(length as usize, length as usize,
                      tuple!(raw_string, le_i32), BTreeMap::new(),
                      |mut classes: BTreeMap<i32, String>, x: (&str, i32)| {
            classes.insert(x.1, x.0.to_owned());
            classes
        }) >>
        (classes)
    )
);

#[derive(Debug)]
pub struct ClassNetCache {
    object_index: i32,
    parent_id: i32,
    id: i32,
    properties: BTreeMap<i32, i32>
}

named!(pub class_net_cache_array <Vec<ClassNetCache>>,
    do_parse!(
        caches: length_count!(le_u32, do_parse!(
            object_index: le_i32 >>
            parent_id: le_i32 >>
            id: le_i32 >>
            p_len: le_i32 >>

            // Apply tuple!(le_i32, le_i32), p_len number of times.
            // Then insert these into a created BTreeMap
            props: fold_many_m_n!(p_len as usize, p_len as usize,
                                  tuple!(le_i32, le_i32), BTreeMap::new(),
                                  |mut props: BTreeMap<i32, i32>, x: (i32, i32)| {
                props.insert(x.0, x.1);
                props
            }) >>
            (ClassNetCache {
                object_index: object_index,
                parent_id: parent_id,
                id: id,
                properties: props
            })
        )) >>
        (caches)
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
    raw_network_stream: RawNetworkStream<'a>,
    debug_strings: Vec<DebugString<'a>>,
    tick_marks: Vec<TickMark<'a>>,
    packages: Vec<&'a str>,
    objects: Vec<&'a str>,
    names: Vec<&'a str>,
    class_indexes: BTreeMap<i32, String>,
    class_net_cache: Vec<ClassNetCache>
}

named!(pub get_body<(ReplayBody)>,
    do_parse!(
        levels: length_count!(le_u32, raw_string) >>
        keyframes: keyframe_array >>
        raw_network_stream: get_raw_netstream >>
        debug_strings: debug_strings_array >>
        tick_marks: tick_marks_array >>
        packages: length_count!(le_u32, raw_string) >>
        objects: length_count!(le_u32, raw_string) >>
        names: length_count!(le_u32, raw_string) >>
        class_indexes: class_index_array >>
        class_net_cache: class_net_cache_array >>

        (ReplayBody {
            levels: levels,
            keyframes: keyframes,
            raw_network_stream: raw_network_stream,
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