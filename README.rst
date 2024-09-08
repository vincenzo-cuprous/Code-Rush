Code Rush
=========
           It will make code runner, like vscode, inside your terminal. And can be usefull on sublime text editor.

Dependencies
=============

- Names

     rustup or rustc

- For Arch

.. code-block:: bash

     sudo pacman -Sy &&
     sudo pacman -S rustup --needed --noconfirm &&
     rustup defualt stable

- For Debian Or Ubuntu

.. code-block:: bash

    sudo apt install rustup

- For Fedora

.. code-block:: bash

   sudo dnf install rustup

Build
=====

.. code-block:: bash

     $ cd coderush-rust &&
     $ cargo build && 
     $ cargo run


Installation
============

.. code-block:: bash

    cd coderush-rust &&
    cd target &&
    cd release &&
    sudo cp coderush /usr/local/bin -r
