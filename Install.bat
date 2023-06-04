@echo off

REM Define the installation directory
set "install_dir=C:\Program Files\terminal-todo"

REM Define the binary file name
set "binary_file=target/release/terminal-todo.exe"

REM Create the installation directory if it doesn't exist
mkdir "%install_dir%"

REM Move to the binary to the installation directory
copy "%binary_file%" "%install_dir%"

REM Add the installation directory to the PATH
setx /m PATH "%install_dir%;%PATH%"

REM Success message
echo Installation has been successful.

