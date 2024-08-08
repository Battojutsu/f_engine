/*
    f_engine - A farming and village life game game engine.
    Copyright (C) 2024  BattoJutsu

    Licensed under GNU AGPLv3
*/

let d = new Dialog();
d.addHeading("Configure keys");
d.addNewRow();
let wkey = d.addTextInput("w", "w");

d.addNewRow();
let akey = d.addTextInput("a", "a");

d.addNewRow();
let skey = d.addTextInput("s", "s");

d.addNewRow();
let dkey = d.addTextInput("d", "d");



var tool = tiled.registerTool("PlaceRectangles", {
    name: "f engine",

    activated: function() {
        d.show();
    },
    deactivated: function() {
        d.reject();
    }
})
