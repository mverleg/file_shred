
File encrypt/decrypt
===============================

Command line utility that encrypts and decrypts files.

Functionality:

* Encryption and decryption using established algorithms.
* Compression.
* Key stretching.
* Salts.
* Checksums.
* Backward-compatibility.
* Pass keys by prompt, argument, environment, file or pipe.
* Warnings for weak keys.

Note that:

* Encrypting the same file twice will give different results, due to salts. This may be a problem for version control.
* When hashing multiple files, they share the same salt. This choice was made because stretching takes long, and because if one key were to be found somehow, it would work for all files regardless of salts.

Future plans:

* Recursively encrypt and decrypt all files in a directory.
* Add a flag to use file's last modified date as salt, so that encrypting twice gives the same result.
* Give the encrypted / decrypted file the same owner and permissions as the original.

