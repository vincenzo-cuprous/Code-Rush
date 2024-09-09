Code Rush
=========
                    It will make code runner, like vscode, inside your terminal. And can be usefull on sublime text editor.

Dependencies
=============

- Names

     1. rustup or rustc
     2. mingw64-gcc
     3. x86_64-pc-windows-gnu

- On Arch

.. code-block:: bash

     sudo pacman -Sy
     sudo pacman -S rustup --needed --noconfirm
     rustup default stable
     rustup target add x86_64-pc-windows-gnu
     sudo pacman -S mingw-w64-gcc


- On Debian Or Ubuntu

.. code-block:: bash

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    rustup target add x86_64-pc-windows-gnu
    sudo apt-get install gcc-mingw-w64



- On Fedora

.. code-block:: bash

   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup target add x86_64-pc-windows-gnu
   sudo dnf install mingw64-gcc



- For Windows

.. code-block:: powershell

    rustup toolchain install stable-x86_64-pc-windows-gnu
    rustup default stable-x86_64-pc-windows-gnu
    rustup override set stable-x86_64-pc-windows-gnu

Build
=====

- On Linux

.. code-block:: bash

     cargo build --release
     cargo build --release --target x86_64-pc-windows-gnu # For Windows

- On Windows

.. code-block:: powershell

    cargo build --release

Installation
============

- For Linux

.. code-block:: bash

        sudo cp coderush /usr/local/bin/ -r

- For Windows

       1. First hit setup.exe
       2. Then open path.exe

- Terminus Plugin For Both Windows and Linux

.. code-block:: json

         [
       // Toggle the default shell in panel
       { "keys": ["ctrl+alt+`"], 
        "command": "terminus_open",
        // "command": "open_terminal_project_folder",
          "args": {
            "shell_cmd": "fish",
            "cwd": "${file_path:${folder}}",
            // "panel_name": "Terminus",
            // "pre_window_hooks": [],
            // "post_window_hooks": [],
            // "default_title": "Coderush",
            // "title": "Run Coderush",
            // "auto_close": false
          }
        },


        // Open a terminal tab at the current file directory and run coderush
         {
        "keys": ["ctrl+alt+space"],
        "command": "toggle_terminus_panel",
        "args": {
            "shell_cmd": "fish",
            "cwd": "${file_path:${folder}}",
            
                }
         }
        ]

