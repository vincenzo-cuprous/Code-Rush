Code Rush
=========
           It will make code runner, like vscode, inside your terminal. And can be usefull on sublime text editor.

Dependencies
=============

- Names

     rustup or rustc

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


Installation
============

.. code-block:: bash

    cd coderush-rust &&
    cd target &&
    cd release &&
    sudo cp coderush /usr/local/bin -r
