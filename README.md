# Tuisic
Experimental TUI music player
## Instructions
### Navigate
To navigate the UI simply use arrow keys. 
#### Arrow up
will select entry above current selection. 
#### Arrow down
will select entry below current selection.
#### Arrow right 
will enter directory *or* try to play file (Note this currently panics if the file cannot be played).
#### Arrow left 
will exit current folder into parent folder. Continue to press left arrow until you've "left" the root folder to exit the program.

Currently no pausing, queuing, etc. Mainly created to experiment with file system and media streaming in Rust, not as a viable media player.
