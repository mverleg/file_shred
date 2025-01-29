
File shred
===============================

Command line utility that safely deletes files.

Functionality
-------------------------------

* Repeatedly overwrite the file content with random data and specific patterns.
* Rename repeatedly to hide that the file ever existed.
* Remove access- and modification time.
* Truncate then delete the file.

In Docker
-------------------------------

Run the shredder with Docker:

    docker run --rm -it -v "$(pwd):/data" mverleg/file-shred -- file.txt

You can mount any directory in which you want to shred files; the above example uses the current directory `$(pwd)`.

To build the image yourself (instead of downloading from Dockerhub), clone the Github project and run:

    docker build -t mverleg/file-shred .

This will also run the tests and lints, to verify that your version is okay.

As binary
-------------------------------

You can shred files like

    shred file.txt image.png /tmp/stuff/*

There are command line options for various things::

    Arguments:
    <FILES>...  One or more paths to input files (absolute or relative)
    
    Options:
    -y, --no-confirm
            Delete files without asking for confirmation
    -v, --debug
            Show debug information, especially on errors
    -q, --quiet
            Do not show progress or other non-critical output
    -k, --keep
            Destroy the data, but do not rename or delete the file. Useful for non-regular files like special system devices
    --overwrite-count <OVERWRITE_COUNT>
            Number of times the file is overwritten (at least 1) [default: 10]
    --rename-count <RENAME_COUNT>
            Number of times the file is renamed

As library
-------------------------------

To use default options on a single file, ues `shred_file`. To customize:

* Create an instance of `::file_shred::ShredConfig`, options largely correspond to the command-line ones.
* In particular note `confirmation_prompt=false` if there might not be an interactive session present.
* Call the `::file_shred::shred` function and pass this config.

Keep in mind that:
-------------------------------

*There are no security guarantees, and the author is not a professional security expert. Use at your own risk.*

* Obviously, be careful. The purpose of this tool is to irrecoverably delete data. I cannot help you get data back if you delete it by accident.
* Note that data recovery difficulty depends on the environment (operating system, hard disk formatting, physical medium). For some configurations, overwriting may not work.

This is used by [`file_endec`](https://github.com/mverleg/file_endec).
