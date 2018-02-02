# Another Rocket League Replay Parser
[![Build Status](https://travis-ci.org/smithsps/rl-replay.svg?branch=master)](https://travis-ci.org/smithsps/rl-replay)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Lots of thanks to [jjbott's](https://github.com/jjbott/RocketLeagueReplayParser) and [tfausak's octane](https://github.com/tfausak/octane) implementations.

For another Rust flavored Rocket League parser go check out: nickbabcock's [boxcars](https://github.com/nickbabcock/boxcars) crate.


## Overview

This project is an Rocket League Replay Parser written in Rust.

This project uses the [nom](https://github.com/Geal/nom) library's parser combinators for quick and hopefully easily maintainable parsing.

This library currently **does not** parse the network stream, but this is the next major feature being worked on.