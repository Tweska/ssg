# Static Site Generator
This program can be used to generate static websites from [Markdown][1] files.
If you don't know how to use Markdown I suggest reading the [Markdown Guide][2]!

## Getting Started
After cloning, compiling and adding the executable to the path variable you can
get stared by creating your first SSG project.

Create a new directory first and initialize it:
```
mkdir <project name>
cd <project name>
ssg init
```

Some files and directories are created for you, you can already generate the
initial project using `ssg`. All the Markdown source files live in the `src`
directory and the template file is at `tpl/template.html`. The generated website
will end up in the `out` directory.

## Advanced Options
The following options are available when generating your static site:
```
--clean: Remove leftover files for which the source no longer exists.
--ignore-unchanged: Do not update output if the source file is not updated since generation.
--include-source: Include the Markdown source file in the output.
```

## Where is SSG used?
I currently use SSG to generate [my own static site][3].


[1]: <https://en.wikipedia.org/wiki/Markdown> "Markdown on Wikipedia"
[2]: <https://www.markdownguide.org/>
[3]: <https://2sk.nl/>
