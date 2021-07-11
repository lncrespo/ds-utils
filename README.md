# Disk space utilities

### ⚠ Work in progress

## Overview
So far, this program can search the filesystem for directories larger than a given threshold and display a file size tree.

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
- `human_readable` - Boolean: Whether to display units as human readable (if false, sizes will be displayed in bytes)
- `max_file_size` - Unsigned Integer (64-Bit): Every directory with an equal or larger size (in bytes) will get reported (defaults to 75MB)
- `file_tree` - Boolean: Whether to print a file space tree
- `tree_length` - Unsigned Integer (8-Bit): How many sub entries per reported directory are shown in the file tree
- `tree_depth` - Unsigned Integer (8-Bit): How many levels of recursion the tree should display. Basically, how deep the tree will loop.

## Disclaimer

This program has only been tested on Linux. There is no guarantee of the program being fully operational on other platforms.
