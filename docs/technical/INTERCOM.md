# intercom

Sophixer uses a specific communication protocol between programs in order to account for low access to languages, for instance with Renoise's plugins.

## Rust

In Rust, the `intercom` crate is here to help. Look at the crate examples, or the `sophixer-core` crate.  
You may only use UDP at the moment.

## Lua

In Lua, you cannot use serializing, so a simple custom format is used: `msg,arg,arg,arg...;msg,arg...;...;`
