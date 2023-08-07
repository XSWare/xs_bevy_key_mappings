# xs_bevy_key_mappings

does not work with stable yet, due to [macro_metavar_expr](https://github.com/rust-lang/rust/issues/83527).

compact key mappings for bevy.  
the following generator macros are available for their respective input interactions:  
```
map_clear_just_pressed
map_clear_just_released
map_just_pressed
map_just_released
map_pressed
```

### example usage
```
map_just_pressed!(
    should_jump => Space,
    should_save => S,
);

fn jump() {
    // your jump code
}

fn save() {
    // your save code
}
```

the generated functions can then be used with bevys `run_if`
```
app
    .add_systems(Update, jump.run_if(should_jump))
    .add_systems(PreUpdate, save.run_if(should_save));
```
