To jot down notes I have about designs. 

# Firmware event/callback/behavior

Maybe try separating behaviors into different types:
1. Standard - one callback for before and one callback for after
2. Event-driven - has a callback that is automatically called on certain events like a scancode pressed

Event - Make the output of any behavior function call an event. That is, keypress and release, layer swap, or anything else. 

# Different keyboards
I think the easiest way is to heavily utilize crates.

## Controllers
Each type of microcontroller can have its own crate, such as `dmk_rp2040`. This crate has any necessary controller dependencies
as well as definitions to enable timing functionality, etc. They also define an interface for the hardware which board crates
can use. 

## Boards
These represent the rest of the configuration for a full keyboard, and would be called something like `dmk_planck` or
`dmk_glove80`. They would define the key matrix using the pins defined by the controller. They would also define what, if any,
peripherals this keyboard has and give the pins for those as well. It would expose some method like `run_from_config`.

## Config
To create a configuration for a `crkbd` board you would create a project which pulls in the `dmk_crkbd` crate (which in turn
pulls in the `dmk_rp2040` crate), and has an `rp2040` build target. You would create your config (probably using macros) and
have your main method call the `run_from_config` method with the config. 
