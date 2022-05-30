# SovietWM - Configuration Documentation

**Please note:** This may be out of date

## Adding custom keybindings:

After line 20 in main.rs, you will need to add the program you wish to launch. Here is an example:

```rs
const STEAM: &str = "steam";
```

This will provide a constant that may be used to open Discord; following that, create the keybinding after line 63. Here is an example:

```rs
"M-h" => run_external!(STEAM);
```

This instructs the Window Manager to start Steam. The capital "M" stands for MOD, which is the default SUPER key. The lower-case "h" represents your keyboard's H key.


## Utilising your changes

Then to use this, you will need to recompile SovietWM [See installation documentation](https://theholytachanka.github.io/SovietWM/install)
