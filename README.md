# miniline

`miniline` is a [powerline](https://github.com/powerline/powerline)-like
command prompt for bash written in [Rust](https://www.rust-lang.org/).

## Installation

`miniline` depends on powerline fonts [check out it's wiki for installation details](https://powerline.readthedocs.org/en/latest/installation.html#fonts-installation).
It also needs to be compiled:

    cargo build --release
    
And the `target/release/miniline` placed inside some `$PATH` directory.
After this, you can activate it adding this to your `.bashrc`

    # Create a function that generates the PS1 using miniline 
    function update_prompt {
        PS1="`miniline $?`"
    }
    # Declare the function as the prompt updating one
    export PROMPT_COMMAND=update_prompt
