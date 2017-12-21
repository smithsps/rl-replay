use nom::*;
use header::primitives::*;

#[derive(Debug)]
pub struct Highlight<'a> {
    frame: u32,
    car_name: &'a str,
    ball_name: &'a str
}

named!(pub highlight_array <Vec<Highlight>>,
    do_parse!(
        tag!(b"\x0B\x00\x00\x00HighLights\0") >>
        array_size: array_property >>

        highlights: length_count!(le_u32, do_parse!(
            tag!(b"\x06\x00\x00\x00frame\0") >>
            frame: int_property >>

            tag!(b"\x08\x00\x00\x00CarName\0") >>
            car_name: name_property >>

            tag!(b"\x09\x00\x00\x00BallName\0") >>
            ball_name: name_property >>

            // Unknown why there is a 'None' delimiting
            str_none >>

            (Highlight {
                frame: frame,
                car_name: car_name,
                ball_name: ball_name
            })
        )) >>
        (highlights)
    )
);