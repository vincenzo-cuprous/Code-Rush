Code Rush
=========
           It will make code runner, like vscode, inside your terminal. And can be usefull on sublime text editor.

Dependencies
=============

- Names

     1.rustup or rustc

- For Arch

.. code-block:: bash

     sudo pacman -Sy
     sudo pacman -S rustup --needed --noconfirm
     rustup default stable

- For Debian Or Ubuntu

.. code-block:: bash

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

- For Fedora

.. code-block:: bash

   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

- For Windows

     

Build
=====

.. code-block:: bash

     cd coderush-rust &&
     cargo build && 
     cargo run
