# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### To Do
- Write function to setup Tenma device  

### Unfinished Ideas

## [0.1.4] - 2021-03-14

### Changed
- The `tenma_data_buffer` variable now implemented as a vector instead of an array
- Overhaul of the main logic
- Interrupted communications now results in outputting error codes (in first character of data output) instead of panic

### Fixed
- Removed left over debug eprintln statements

### Removed
- All newline logic is removed. Newline characters are truncated and instead we use println! to print newline

## [0.1.3] - 2021-03-13

### Added
- We now panic if no valid devices are found
- We now have the -t option that prepends the data string with a time stamp. By default it will print the date and time, but the user can specify their own format in accordance with the `chrono:format::strftime` format.
- Native binary to sourceforge
- newline option to modify the newline character(s)
- Added device option to select Tenma device
- Validator for custom date/time formats

## [0.1.2] - 2021-03-13

### Added
- added code to configure endpoint
- Parity detection
- Verbosity command line option

### Fixed
- Various warnings cleared

## [0.1.1] - 2021-03-13

### Added
- Code to talk to Tenma over USB and obtain 11-byte data packet

## [0.1.0] - 2021-03-13

Initial Commit

### Added
- Basic system to obtain a list of USB devices
- System to recognise Tenma USB volt meter (based on specific product and vendor ID)