use header::primitives::*;

#[derive(Debug)]
pub struct MetaInfo<'a> {
    replay_name: Option<&'a str>,
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
    creator_player_name: &'a str
}

named!(pub meta_info <MetaInfo>,
    do_parse!(
        opt!(tag!(b"\x0B\x00\x00\x00ReplayName\0")) >>
        replay_name: opt!(str_property) >>

        tag!(b"\x0E\x00\x00\x00ReplayVersion\0") >>
        replay_version: int_property >>

        tag!(b"\x0C\x00\x00\x00GameVersion\0") >>
        game_version: int_property >>

        tag!(b"\x08\x00\x00\x00BuildID\0") >>
        build_id: int_property >>

        tag!(b"\x0B\x00\x00\x00Changelist\0") >>
        changelist: int_property >>

        tag!(b"\x0D\x00\x00\x00BuildVersion\0") >>
        build_version: str_property >>

        tag!(b"\x0A\x00\x00\x00RecordFPS\0") >>
        record_fps: float_property >>

        tag!(b"\x0E\x00\x00\x00KeyframeDelay\0") >>
        keyframe_delay: float_property >>

        tag!(b"\x0C\x00\x00\x00MaxChannels\0") >>
        max_channels: int_property >>

        tag!(b"\x10\x00\x00\x00MaxReplaySizeMB\0") >>
        max_replay_size_mb: int_property >>

        tag!(b"\x03\x00\x00\x00Id\0") >>
        id: str_property >>

        tag!(b"\x08\x00\x00\x00MapName\0") >>
        map_name: name_property >>

        tag!(b"\x05\x00\x00\x00Date\0") >>
        date: str_property >>

        tag!(b"\x0A\x00\x00\x00NumFrames\0") >>
        num_frames: int_property >>

        tag!(b"\x0A\x00\x00\x00MatchType\0") >>
        match_type: name_property >>

        tag!(b"\x0B\x00\x00\x00PlayerName\0") >>
        player_name: str_property >>

        str_none >>

        (MetaInfo {
            replay_name: replay_name,
            replay_version: replay_version,
            game_version: game_version,
            build_id: build_id,
            changelist: changelist,
            build_version: build_version,
            record_fps: record_fps,
            keyframe_delay: keyframe_delay,
            max_channels: max_channels,
            max_replay_size_mb: max_replay_size_mb,
            id: id,
            map_name: map_name,
            date: date,
            num_frames: num_frames,
            match_type: match_type,
            creator_player_name: player_name
        })
    )
);