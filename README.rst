
File shred (Rust)
===============================

Command line utility that safely deletes files.

Functionality:

* Repeatedly overwrite the file content with random data and specific patterns.
* Rename repeatedly to hide that the file ever existed.
* Remove access adn modification date.

.. note :: While this mostly relies on established hashing and encryption algorithms, there are no security guarantees, and the author is not a professional security expert. Use at your own risk.

The code can be used as a binary (`shred -h` for info) or as a library.

This was split off from the encryption util file_endec_, which still uses it.

.. _file_endec: https://github.com/mverleg/file_endec

