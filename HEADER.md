
### Header Structure:

<Header-Length><CRC><Header-Data> (Stats and Meta data)
<Body-Length><CRC><Body-Data> (Or rather Match/Event/Netdata/etc.)



```
<4 bytes - Header Length>
<4 bytes - Header CRC>

<4 bytes - Engine Version>
<4 bytes - License Version>
<4 bytes - Patch Version  - (Only if Version Major >= 868 and Minor >= 18)>
<string TAGame.Replay_Soccar_TA>

<IntProperty TeamSize>
<IntProperty PrimaryPlayerTeam>
<IntProperty Team0Score>
<IntProperty Team1Score>

Goals Array:
Goal Object {
    <IntProperty Frame>
    <StrProperty Name>
    <IntProperty Team>
    <Delimiting None String>
}

Highlights Array:
Highlight Object {
    <IntProperty Frame>
    <StrProperty Car_Name>
    <StrProperty Ball_Name>
}

Player Stats Table:
Player_Stats Object {
    <StrProperty Player_Name>
    <ByteProperty Platform>
    <RawString OnlinePlatform>
    <QWordProperty OnlineID>
    <IntProperty Team>
    <IntProperty Score>
    <IntProperty Goals>
    <IntProperty Assists>
    <IntProperty Saves>
    <IntProperty Shots>
    <BoolProperty Bot>
}

Meta Information Table:
Meta Table {
    <StrProperty Replay_Name - (Optional)>
    <IntProperty Replay_Version>
    <IntProperty Game_Version>
    <IntProperty Build_ID>
    <IntProperty Changelist>
    <StrProperty Build_Version>
    <FloatProperty Record_FPS>
    <FloatProperty Keyframe_Delay>
    <IntProperty Max_Channels>
    <IntProperty MaxReplaySizeMB>
    <StrProperty ID>
    <NameProperty Map_Name>
    <StrProperty Date>
    <IntProperty Total_Frames>
    <NameProperty Match_Type>
    <StrProperty Recording_Player_Name>
}
```

