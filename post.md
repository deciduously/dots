# Code dive - dots

This is another code dive, a la [Stalk a Click through a Re-Frame/actix_web App](https://dev.to/deciduously/stalk-a-click-through-a-re-frameactixweb-app-55fb).  I'm choosing a function and exploring how it does its mojo.  The example code we're diving through is [playable](https://http://deciduously.com/dots), and the specific function we're gonna trace is is the click to place a dot and start a level.  As this is a rather passive click is which that's most of the user interaction, this will end up pulling us through a solid chunk of the code.

The full repo can be found [here](https://github.com/deciduously/dots)