#[macro_use]
extern crate nom;

use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str::from_utf8;
use nom::*;

static REPLAY_FILE_STR: &'static str = "replays/DB55DE0C49529D55D2D1E0B024869CF3.replay";
//static REPLAY_FILE_STR: &'static str = "replays/F32599A54B1831A58C6C55A5334890AF.replay";
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
        string: take_str!(length - 1) >>
        null_terminator: take!(1) >>
        (string)
    )
);

named!(bool_property<bool>,
    do_parse!(
        tag!(b"\x0D\x00\x00\x00BoolProperty\0") >>
        boolean: le_u64 >>
        null_terminator: take!(1) >>
        (boolean > 0)
    )
);

named!(name_property<&str>,
    do_parse!(
        tag!(b"\x0D\x00\x00\x00NameProperty\0") >>
        property_size: le_u64 >>
        length: le_u32 >>
        string: take_str!(length - 1) >>
        null_terminator: take!(1) >>
        (string)
    )
);

named!(byte_property,
    do_parse!(
        tag!(b"\x0D\x00\x00\x00ByteProperty\0") >>
        bytes: take!(8) >>
        (bytes)
    )
);

named!(qword_property,
    do_parse!(
        tag!(b"\x0E\x00\x00\x00QWordProperty\0") >>
        property_size: le_u64 >>
        bytes: take!(8) >>
        (bytes)
    )
);

named!(raw_string<&str>,
    do_parse!(
        length: le_u32 >>
        string: take_str!(length - 1) >>
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

        highlights: length_count!(le_u32, do_parse!(
            str_frame >>
            frame: int_property >>

            str_CarName >>
            car_name: name_property >>

            str_BallName >>
            ball_name: name_property >>
            
            // Unknown why there is a 'None' delimiting
            str_None >> 

            (Highlight {
                frame: frame,
                car_name: car_name,
                ball_name: ball_name
            })
        )) >>
        (highlights)
    )
);

#[derive(Debug)]
struct PlayerStats<'a> {
    name: &'a str,
    is_bot: bool,
    platform: &'a [u8],
    online_platform: &'a str,
    online_id: &'a [u8],
    team: u32,
    score: u32,
    goals: u32,
    assists: u32,
    saves: u32,
    shots: u32
}

named!(playerstats_array <Vec<PlayerStats>>,
    do_parse!(
        tag!(b"\x0C\x00\x00\x00PlayerStats\0") >>
        array_size: array_property >>

        stats: length_count!(le_u32, do_parse!(
            tag!(b"\x05\x00\x00\x00Name\0") >>
            player_name: str_property >>

            tag!(b"\x09\x00\x00\x00Platform\0") >>
            platform: byte_property >>

            tag!(b"\x0F\x00\x00\x00OnlinePlatform\0") >>
            online_platform: raw_string >>

            tag!(b"\x09\x00\x00\x00OnlineID\0") >>
            online_id: qword_property >>

            tag!(b"\x05\x00\x00\x00Team\0") >>
            team: int_property >>

            tag!(b"\x06\x00\x00\x00Score\0") >>
            score: int_property >>

            tag!(b"\x06\x00\x00\x00Goals\0") >>
            goals: int_property >>

            tag!(b"\x08\x00\x00\x00Assists\0") >>
            assists: int_property >>

            tag!(b"\x06\x00\x00\x00Saves\0") >>
            saves: int_property >>

            tag!(b"\x06\x00\x00\x00Shots\0") >>
            shots: int_property >>

            tag!(b"\x05\x00\x00\x00bBot\0") >>
            is_bot: bool_property >>

            str_None >>

            (PlayerStats {
                name: player_name,
                is_bot: is_bot,
                platform: platform,
                online_platform: online_platform,
                online_id: online_id,
                team: team,
                score: score,
                goals: goals,
                assists: assists,
                saves: saves,
                shots: shots
            })
        )) >>
        (stats)
    )
);

#[derive(Debug)]
struct MetaInfo<'a> {
    replay_version: u32,
    game_version: u32,
    build_id: u32,
    changelist: u32,
    build_version: &'a str,
    record_fps: f32,
    keyframe_delay: f32,
    max_channels: u32,
    max_replay_size_mb: u32,
    id: &'a str,
    map_name: &'a str,
    date: &'a str,
    num_frames: u32,
    match_type: &'a str,
    online_player_name: &'a str,
}

named!(meta_info <Vec<MetaInfo>>,
    do_parse!(
        rest: rest >>
    )
);

named!(get_header<(Vec<PlayerStats>)>,
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

        stats: playerstats_array >>

        rest: rest >>
        (stats)
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