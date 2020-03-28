
File shred (Rust)
===============================

Command line utility that safely deletes files.

Functionality:

* Repeatedly overwrite the file content with random data and specific patterns.
* Rename repeatedly to hide that the file ever existed.
* Remove access adn modification date.

_There are no security guarantees, and the author is not a professional security expert. Use at your own risk._

Keep in mind that:

* Obviously, be careful. The purpose of this tool is to irrecoverably delete data. I cannot help you get data back if you delete it by accident.
* Note that data recovery difficulty depends on the environment (operating system, hard disk formatting, physical medium). For some configurations, overwriting may not work.

The code can be used as a binary (`shred -h` for info) or as a library.

This was split off from the encryption util file_endec_, which still uses it.

.. _file_endec: https://github.com/mverleg/file_endec

