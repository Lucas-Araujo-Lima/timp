# What is it and why

timp (short for "template import") is a simple cli utility for importing code templates from the ~/Templates directory.
it's goal is to help skip writing common boilerplate on new files, using simple to make templates.


# Usage

Say you have a template named "clib" in ~/Templates. In that template, you also have a "libname" macro that you want to replace with the library's name when you import it.
Then, to do that, you need only run "timp clib output_file -libname=name" and timp will do all the replacement for you.

In a more complex scenario, your template may have multiple macros, that can be defined with the same "-macro=definition" pattern. It is, however, important to know,
when writing a template, that timp only does a simple text replacement. Therefore, if the text replacement breaks the language's syntax, timp will not correct it.

Also, if the user does not give a definition to a macro used in a template, timp simply does not do any replacement, so the output will have the macros still in it.
Templates can, however, give default definitions to macros that are applied if the user does not provide one.
