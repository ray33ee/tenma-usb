# TenmaUSB

A simple application the prints the output of Tenma volt meters. 

Tested on Tenma 72-7732A.

# Output

Without any command line options, a successful read from the device will print 12 characters.
The first character is an ascii '0' or '1' indicating the success or fail, respectively of the parity check for the received data.
The next 11 characters are the data from the device.

# Options

## Newline

By default the last two aharacters are always a CRLF terminator. However this can be modified to be unix compatible with the '-n="unix"' option.

## Time stamp

Without this option no time stamp is used. However using -t will prepend the data with a date/time stamp.
The user can specify a custom format with '-t="%H:%M:%S"'. The formatting is as defined by the `chrono:format::strftime` Rust module.

## Device

If multiple Tenma devices must be used by TenmaUSB, the exact device can be specified using a number.

## Verbosity

If -v is used, extra information can be displayed to the user (outputted to `stderr`)