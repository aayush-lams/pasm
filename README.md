# pasm
A minimal credential manager
## How to use
pasm is minimal and thus really simple to use. 
  - use `pasm write .` to start writing a new entry. It creates a new 'pasmuser0.txt' file in '~' if not already there and writes into it
  - use `pasm display .` to display all the entries inside the 'pasmuser0.txt' file
  - use `pasm find [entry_name]` to find and display content of particular entry. For eg: `pasm find github` displays entry content with 'discord' name
  - use `pasm edit [entry_name]` to edit particular entry
  - use `pasm remove [filename]` to remove current file. For eg `pasm remove pasmuser0.txt` removes the 'pasmuser0.txt' file
  - use `pasm delete [entry_name]` to remove particular entry
  - use `pasm help .` to display list of command
