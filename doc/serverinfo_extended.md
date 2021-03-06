In the following document, "vanilla" refers to unmodified Teeworlds.


Extended server info request
============================

An extended server info can be requested from the Teeworlds server by sending
the following UDP packet:

    [2] magic_bytes
    [2] extra_token
    [2] reserved
    [4] padding
    [4] vanilla_request
    [1] token

`magic_bytes` must be the ASCII representation of "xe".

`extra_token` contains a 16 bit big-endian integer that should be shifted by 8
to the left and combined with `token` with a bitwise or, before the extended
server info is sent back.

`reserved` is reserved for future protocol versions and must be zeroed by the
sender and ignored by the receiver with this protocol version.

`padding` must be filled with 0xff bytes.

`vanilla_request` must be the ASCII representation of "gie3".

`token` is an arbitrary byte chosen by the client which will be sent back with
the server info. For the extended server info, the `extra_token` field needs to
be included in the server info response.

If a server receives such a request (detected by the `magic_bytes` field), it
can either respond with the vanilla server info, or by sending the extended
server info, described in the following.

If the server does not recognize the `magic_bytes`, it should send back the
vanilla server info.


Extended server info
====================

The extended server info can consist of several packets, the "main" packet and
the "more" packets, which can contain additional player data in case they don't
fit in the first packet.

The `str` type is a zero-terminated UTF-8 string. Implementations should be
prepared to either replace invalid UTF-8 with the replacement character or to
treat these fields as opaque zero-terminated byte sequences.

The `int` type is an integer encoded in the decimal system in a zero-terminated
ASCII string.

NOTE: The reference implementation clears control characters (replacing bytes
with a value of less than 32 with the ASCII space ' ') and skips leading
whitespace in all strings.

The receiver of such a server info can detect whether it has received the
server info completely by comparing the number of players received with the
number of players set in the main packet.


The main packet
---------------

The main server info packet looks like follows:

    [ 10] padding
    [  4] type
    [int] token
    [str] version
    [str] name
    [str] map
    [int] map_crc
    [int] map_size
    [str] game_type
    [int] flags
    [int] num_players
    [int] max_players
    [int] num_clients
    [int] max_clients
    [str] reserved
    [  *] players

`padding` must be filled with 0xff bytes.

`type` must be the ASCII representation of "iext".

`token` must be the token from the request. If the server info isn't sent upon
a request, it mut be -1.

`version` contains the server version. The first three bytes must equal the
client's version, otherwise it will be filtered out as "incompatible version".

`name`, `map`, `game_type` are the server's name, its current map and gametype,
respectively. `map_crc` is the current map's CRC (note that Teeworlds uses
incorrect starting values to calculate the CRC) and `map_size` is the map's
size in bytes.

There is currently only one flag for the `flags` field, namely the password
flag, whose value is 1. It must be set if entering the server requires a
password, and unset in the other case.

`num_players`, `max_players`, `num_clients`, `max_clients`: These values
describe how many players (people that are not in "spectator mode") and clients
(people that are connected to the server) the server currently has (`num_*`)
and is able to handle (`max_*`). The following natural inequalities must hold:

                     <= max_players
    0 <= num_players                <= max_clients
                     <= num_clients

Note that there is no upper limit for `max_clients` enforced by the protocol.

The field `reserved` is reserved for future protocol versions. Senders of this
message must set the field to the empty string, and receivers of this message
must ignore its value.

`players` contains the player info described further below.


The "more" packet
-----------------

A "more" packet looks like this:

    [ 10] padding
    [  4] type
    [int] token
    [int] packet_no
    [str] reserved
    [  *] players

`padding`, `token`, `reserved`, `players`: See the explanation on the main
packet.

`type` must contain the ASCII representation of "iex+".

`packet_no` must be packet number in the server info response. It must be
greater than 0 and less than 64. The main packet has an implicit `packet_no` of
0. This field can be used to detect duplicated packets.


Player info
-----------

A single player info is defined as:

    [str] name
    [str] clan
    [int] country
    [int] score
    [int] is_player
    [str] reserved

The `name`, `clan` and `score` field contain the player's name, clan and score,
respectively.

The `country` field contains the ISO 3166-1 numeric code of the player's
country, or -1 if unset.

The `is_player` field contains a non-zero integer if the player is not a
spectator, and 0 otherwise. Producers of this message should always use 1 to
indicate a non-spectator client.
