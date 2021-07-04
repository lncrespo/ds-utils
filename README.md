# Disk space utilities

### ⚠ Work in progress

## Overview
So far, this program can search the filesystem for directories larger than a given threshold.

`ds-utils` will look for a configuration file named `ds-utils.toml` under your configuration directory:

Linux: `/home/{username}/.config/ds-utils.toml`  
macOS: `/Users/{username}/Library/Preferences/ds-utils.toml`  
Windows: `C:\Users\{username}\AppData\Roaming\ds-utils.toml`  

## Configuration
A default configuration file is located in the repository root.

To use a different configuration file, simply pass in the path to the configuration file as second parameter:  
`ds-utils /my/custom/config.toml`

Possible entries:
- `directories` - Array: Used to hold every directory you want to check
- `max_file_size` - Integer: Every directory with an equal or larger size (in bytes) will get reported (defaults to 75MB)
