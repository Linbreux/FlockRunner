# Where should we write the GIF?
Output example.gif

# Set up a 1200x600 terminal with 46px font.
Set FontSize 18
Set Width 1200
Set Height 700
Set Shell zsh

##########
Type "fr"

Sleep 1s
Enter
Sleep 4s

##########
Hide
Type "clear"
Enter
Show

Type "fr list"
Sleep 1s
Enter
Sleep 4s


##########
Hide
Type "clear"
Enter
Show

Type "fr cmd --help"
Sleep 1s
Enter
Sleep 4s

##########
Hide
Type "clear"
Enter
Show

Type "fr cmd hello --dryrun"
Sleep 1s
Enter
Sleep 4s
Enter

Type "fr cmd hello -v"
Sleep 1s
Enter
Sleep 4s
Enter

Type "fr cmd hello -D user=Linbreux"
Sleep 1s
Enter
Sleep 4s

##########
Hide
Type "clear"
Enter
Show

Type "fr cmd create"
Sleep 1s
Enter
Sleep 4s

Enter

Type "fr cmd create"
Sleep 1s
Enter
Sleep 4s

Enter

Type "ls flock"
Sleep 1s
Enter
Sleep 1s

Enter

Type "fr cmd clean -v"
Sleep 1s
Enter
Sleep 4s

Enter

Type "ls flock"
Sleep 1s
Enter
Sleep 1s

##########
Hide
Type "clear"
Enter
Show

Type "fr cmd run_on_os -v"
Sleep 1s
Enter
Sleep 4s

Enter

Type "fr cmd run_on_os -D os=ubuntu -v"
Sleep 1s
Enter
Sleep 4s

Enter
