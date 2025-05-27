# watch_file

Watches a file and runs it when
it changes.

## Usage

wf <file_path>

## Notes

- DONE: Uses `bash` to execute the file. 

- DONE: Does a `cd` into the file's directory
if it's not in the current one.

- DONE: Does not execute the file when you first
start it up. (I went back and forth on this
but ended up with not doing the first run. 
The idea being that the app runs the same
way all the time. That is, it only 
runs when a change has occurred)

- DONE: Clears the terminal on start and
prints a `Watching FILE_PATH` 
message.

- DONE: Clears the terminal before each run

- DONE: Prints a small report with a timestamp,
if a CD happened, the name of the file,
and the time it took to run. 

- DONE: Kills the process and restarts it
if a change occurs while its still running. 

- DONE: Passing `-q` or `--quiet` turns off 
the initial `Watching` and ending
report.


## NOTES

- If you set an ENV variable it'll 
be available in the script

- I'm not sure how much padding the
timing check ads in terms of
overhead. If you need something
super accurate you'll want to build
it into your process directly. 


## Someday/Maybe Features

- Figure out how to add a test suite.

- Provide `--no-cd` flag to run from 
the current direction instead of 
CD-ing into the scripts dir if it's
different

- Provide `-g|--glob PATTERN` to get a 
glob pattern of files to watch

- Add a `-s|--shell` to set the shell

- Look at sending a signal and waiting
for process to close down during a
grace period if there's a change
while it's still running. 

