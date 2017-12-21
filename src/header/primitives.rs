// Primitives

use nom::*;

named!(pub str_none, tag!(b"\x05\x00\x00\x00None\0"));

named!(pub int_property<u32>,
    do_parse!(
        tag!(b"\x0C\x00\x00\x00IntProperty\0") >>
        // Always 4 bytes? tag!(b"\x04\x00\x00\x00\x00\x00\x00\x00") >>
        property_size: le_u64 >>
        integer: le_u32 >>
        (integer)
    )
);

named!(pub array_property<u64>,
    do_parse!(
        tag!(b"\x0E\x00\x00\x00ArrayProperty\0") >>
        property_size: le_u64 >>
        (property_size)
    )
);

named!(pub str_property<&str>,
    do_parse!(
        tag!(b"\x0C\x00\x00\x00StrProperty\0") >>
        property_size: le_u64 >>
        length: le_u32 >>
        string: take_str!(length - 1) >>
        null_terminator: take!(1) >>
        (string)
    )
);

named!(pub bool_property<bool>,
    do_parse!(
        tag!(b"\x0D\x00\x00\x00BoolProperty\0") >>
        boolean: le_u64 >>
        null_terminator: take!(1) >>
        (boolean > 0)
    )
);

named!(pub name_property<&str>,
    do_parse!(
        tag!(b"\x0D\x00\x00\x00NameProperty\0") >>
        property_size: le_u64 >>
        length: le_u32 >>
        string: take_str!(length - 1) >>
        null_terminator: take!(1) >>
        (string)
    )
);

named!(pub byte_property,
    do_parse!(
        tag!(b"\x0D\x00\x00\x00ByteProperty\0") >>
        bytes: take!(8) >>
        (bytes)
    )
);

named!(pub qword_property,
    do_parse!(
        tag!(b"\x0E\x00\x00\x00QWordProperty\0") >>
        property_size: le_u64 >>
        bytes: take!(8) >>
        (bytes)
    )
);

named!(pub float_property<f32>,
    do_parse!(
        tag!(b"\x0E\x00\x00\x00FloatProperty\0") >>
        length: le_u64 >>
        float: le_f32 >>
        (float)
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