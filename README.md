# watch_file

Watches a file and runs it when
it changes.

## Usage

wf <file_path>

## Notes

- Uses `bash` to execute the file. 

- Does a `cd` into the file's directory
if it's not in the current one.

- Does not execute the file when you first
start it up. (I went back and forth on this
but ended up with not doing the first run. 
The idea being that the app runs the same
way all the time. That is, it only 
runs when a change has occurred)

- TODO: Clears the terminal on start and
prints a `Watching FILE_PATH` 
message.

- TODO: Clears the terminal before each run

- TODO: Prints a small report with a timestamp,
if a CD happened, the name of the file,
and the time it took to run. 



## Options

- TODO: Passing `-q` or `--quiet` turns off 
the initial `Watching` and ending
report.

## TODO

- CD into the directory with the script
if you're not already in it before
running. 

- Provide --no-cd flag to run from 
the current direction instead of 
CDing into the scripts dir if it's
different

- Provide `-g|--glob PATTERN` to get a 
glob pattern of files to watch

- Add a `-s|--shell` to set the shell


## NOTES

- If you set an ENV variable it'll 
be available in the script


