#[macro_use]
extern crate nom;

use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::from_utf8;
use nom::*;

static REPLAY_FILE_STR: &'static str = "DB55DE0C49529D55D2D1E0B024869CF3.replay";
//static REPLAY_FILE_STR: &'static str = "F32599A54B1831A58C6C55A5334890AF.replay";
const DEFAULT_REPLAY_BUFFER: usize = 5400; // 2MB

/*
named!(rl_string<&str>,
    do_parse!(
        length: le_u32 >>
        string: map_res!(
            take!(length - 1), 
            from_utf8
        ) >>
        skip: anychar >>
        (string)
    )
);*/

// Meta programming to simplfy this?
named!(str_TAGame_Header, tag!(b"\x18\x00\x00\x00TAGame.Replay_Soccar_TA\0"));
named!(str_TeamSize, tag!(b"\x09\x00\x00\x00TeamSize\0"));
named!(str_PrimaryPlayerTeam, tag!(b"\x12\x00\x00\x00PrimaryPlayerTeam\0"));
named!(str_Team0Score, tag!(b"\x0B\x00\x00\x00Team0Score\0"));
named!(str_Team1Score, tag!(b"\x0B\x00\x00\x00Team1Score\0"));
named!(str_PlayerName, tag!(b"\x0B\x00\x00\x00PlayerName\0"));
named!(str_PlayerTeam, tag!(b"\x0B\x00\x00\x00PlayerTeam\0"));


named!(int_property<u32>, 
    do_parse!(
        tag!(b"\x0C\x00\x00\x00IntProperty\0") >>
        // Always 4 bytes? tag!(b"\x04\x00\x00\x00\x00\x00\x00\x00") >>
        property_size: le_u64 >>
        integer: le_u32 >>
        (integer)
    )
);

named!(array_property<u64>, 
    do_parse!(
        tag!(b"\x0E\x00\x00\x00ArrayProperty\0") >>
        property_size: le_u64 >>
        (property_size)
    )
);

named!(str_property<&str>,
    do_parse!(
        tag!(b"\x0C\x00\x00\x00StrProperty\0") >>
        property_size: le_u64 >>
        length: le_u32 >>
        string: map_res!(
            take!(length - 1), 
            from_utf8
        ) >>
        null_terminator: take!(1) >>
        (string)
    )
);

named!(name_property<&str>,
    do_parse!(
        tag!(b"\x0D\x00\x00\x00NameProperty\0") >>
        property_size: le_u64 >>
        length: le_u32 >>
        string: map_res!(
            take!(length - 1), 
            from_utf8
        ) >>
        null_terminator: take!(1) >>
        (string)
    )
);




named!(str_Goals, tag!(b"\x06\x00\x00\x00Goals\0"));
named!(str_frame, tag!(b"\x06\x00\x00\x00frame\0"));
named!(str_None, tag!(b"\x05\x00\x00\x00None\0"));

#[derive(Debug)]
struct Goal<'a> {
    frame: u32,
    player: &'a str,
    team: u32
}

named!(goals_array <Vec<Goal>>, 
    do_parse!(
        str_Goals >>
        array_size: array_property >>

        goals: length_count!(le_u32, do_parse!(
            str_frame >>
            frame: int_property >>

            str_PlayerName >>
            player_name: str_property >>

            str_PlayerTeam >>
            team: int_property >>

            // Unknown why there is a 'None' delimiting
            str_None >> 

            (Goal {
                frame: frame,
                player: player_name,
                team: team
            })
        )) >>
        (goals)
    )
);


named!(str_HighLights, tag!(b"\x0B\x00\x00\x00HighLights\0"));
named!(str_CarName, tag!(b"\x08\x00\x00\x00CarName\0"));
named!(str_BallName, tag!(b"\x09\x00\x00\x00BallName\0"));

#[derive(Debug)]
struct Highlight<'a> {
    frame: u32,
    car_name: &'a str,
    ball_name: &'a str 
}

named!(highlight_array <Vec<Highlight>>, 
    do_parse!(
        str_HighLights >>
        array_size: array_property >>

        highlights: length_count!(le_u32, dbg!(do_parse!(
            str_frame >>
            frame: int_property >>

            dbg!(str_CarName) >>
            car_name: name_property >>

            dbg!(str_BallName) >>
            ball_name: name_property >>
            
            // Unknown why there is a 'None' delimiting
            str_None >> 

            (Highlight {
                frame: frame,
                car_name: car_name,
                ball_name: ball_name
            })
        ))) >>
        (highlights)
    )
);


named!(get_header<(u32, u32, Vec<Goal>, Vec<Highlight>)>,
    do_parse!(
        crc: take!(4) >>
        version_major: le_u32 >>
        version_minor: le_u32 >>
        str_TAGame_Header >>

        str_TeamSize >>
        team_size: int_property >>

        opt!(str_PrimaryPlayerTeam) >>
        primary_player_team: opt!(int_property) >>

        str_Team0Score >>
        team_0_score: int_property >>
        
        str_Team1Score >>
        team_1_score: int_property >>
        
        goals: goals_array >>

        highlights: highlight_array >>

        rest: rest >>
        (team_0_score, team_1_score, goals, highlights)
    )
);


fn parse(file: &str) -> io::Result<()> {
    let mut file = File::open(file)?;
    

    let mut hlen_buf = [0; 4];
    file.read_exact(&mut hlen_buf);
    let (_, mut header_length) = le_u32(&mut hlen_buf).unwrap();
    header_length += 4; // CRC is not included. 

    println!("{:?}", header_length);

    let mut h_buf = BufReader::with_capacity(header_length as usize, file);
    let h_bytes = h_buf.fill_buf()?;

    println!("{:?}", get_header(h_bytes));

    Ok(())
}



fn main() {
    parse(REPLAY_FILE_STR).unwrap();
}