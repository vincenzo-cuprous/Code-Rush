Code Rush
===============
It will make code runner, like vscode, inside your terminal.

Dependencies
=============

- For Arch

.. code-block:: bash

    $ pyinstaller (python module)
    $ python3
    $ rustup or rustc 

Build
=====
- In Python

.. code-block:: bash

      $ pyinstaller --onefile coderush.py

- In Rust

.. code-block:: bash

     $ cargo build && cargo run


Installation
============

.. code-block:: bash

    $ sudo cp coderush /usr/local/bin -r
