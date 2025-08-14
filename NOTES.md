To jot down notes I have about alternate designs. 

Maybe try separating behaviors into different types:
1. Standard - one callback for before and one callback for after
2. Event-driven - has a callback that is automatically called on certain events like a scancode pressed
3. Singleton - one FnOnce callback 

Event - Make the output of any behavior function call an event. That is, keypress and release, layer swap, or anything else. 
