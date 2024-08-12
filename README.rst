Code Rush
=========
It will make code runner, like vscode, inside your terminal.

Dependencies
=============

- Names

.. code-block:: bash


    $ pyinstaller (python module)
    $ python3
    $ rustup or rustc
    $ fish

- For Arch

.. code-block:: bash

   $ sudo bash dependencies_for_python.bash
   $ sudo bash dependencies_for_rust.bash


Build
=====
- In Python

.. code-block:: bash

      $ pyinstaller --onefile coderush.py

- In Rust

.. code-block:: bash

     $ cd coderush-rust &&
     $ cargo build && 
     $ cargo run


Installation
============

- For Python 

.. code-block:: bash

    $ sudo cp coderush /usr/local/bin -r

- For Rust
    
.. code-block:: bash

   $ cd target &&
   $ cd debug &&
   $ sudo cp coderush /usr/local/bin -r
