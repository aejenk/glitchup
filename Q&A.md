# Q & A

For any questions you may have!

## What is *databending?*

[Databending](https://en.wikipedia.org/wiki/Databending) is - in layman's terms - *glitching* a file by modifying its data. How this works is similar in all methods: *the bits of the file are modified*. The following are some examples:

- Applying audio effects to a non-audio file (ex. editing a *JPEG* in *Audacity*)
- Photo editing a file that's not an image (ex. editing an *MP4* in *Photoshop*)
- Modifying the bytes of a file directly (ex. using good ol' *Notepad* or by using a *hex editor*.

## Why did you create `glitchup`, and why should I use it?

I found that other results were pretty manual and slow. Opening files in programs forced you to work one at a time, and sometimes those programs even failed loading the file to begin with. The only methods that always worked were by editing the bytes directly, but that proved to be too slow a method.

I wanted to databend on a larger scale, with enough customizeability and methods that you could get various results with a *single file*. Taking inspiration from the manual byte editing, I decided to make [`BEND++`](https://github.com/Calmynt/BENDPlusPlus) - the precursor to `glitchup`.

At some point I considered `BEND++` finished - and stopped working on it for a while. However after some time I started learning Rust, and decided to remake it - partly to learn more about the language, and partly to try and improve upon `BEND++`, now that I had more experience with making databenders. Currently, these are the improvements:

- *Files are memory mapped rather than loaded into memory.* This leads to much better performance. As an example, if you load an 8GB file and modify 10MB chunks of data at a time, with the old method you'd need to load the whole 8GB file into memory. The new method would only end up using 10MB instead.
- *The options file is vastly superior.* This databender uses [**TOML**](https://github.com/toml-lang/toml) rather than [**INI**](https://en.wikipedia.org/wiki/INI_file) for the format of its options file. Check the [**options**](https://github.com/Calmynt/glitchup/blob/master/README.md#options) section in the README to know more.
- *Performance has an unbelievable boost.* Currently I'm only basing this off of personal experience, but at some point I hope to provide some minor benchmarks.

## What's this about a library?

***TODO***
