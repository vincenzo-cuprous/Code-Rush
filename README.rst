Code Rush
=========
           It will make code runner, like vscode, inside your terminal. And can be usefull on sublime text editor.

Dependencies
=============

- Names

     1.rustup or rustc

- On Arch

.. code-block:: bash

     sudo pacman -Sy
     sudo pacman -S rustup --needed --noconfirm
     rustup default stable

- On Debian Or Ubuntu

.. code-block:: bash

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

- On Fedora

.. code-block:: bash

   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

- For Windows

     

Build
=====

- On Linux

.. code-block:: bash

     cargo build --release

- To Compile For Window

.. code-block:: bash

       cargo build --release --target x86_64-pc-windows-gnu

- On Windows

.. code-block:: powershell

    cargo build --release

Installation
============

- For Linux

.. code-block:: bash

        sudo cp coderush /usr/local/bin/ -r

- For Windows

        Just use it and make fun.
      
